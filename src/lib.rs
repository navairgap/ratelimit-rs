//! A small, in-memory token-bucket rate limiter.
//!
//! The clock is monotonic (Instant) and can be supplied to try_acquire_at
//! for deterministic tests. A rejected acquisition does not consume tokens.
//! This limiter is process-local; it does not coordinate across replicas.

use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    ZeroCapacity,
    ZeroRefillTokens,
    ZeroRefillPeriod,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::ZeroCapacity => "capacity must be greater than zero",
            Self::ZeroRefillTokens => "refill_tokens must be greater than zero",
            Self::ZeroRefillPeriod => "refill_period must be greater than zero",
        };
        f.write_str(message)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug)]
struct State {
    tokens: f64,
    last_refill: Instant,
}

/// Thread-safe token bucket with fractional refill precision.
pub struct TokenBucket {
    capacity: f64,
    refill_tokens: f64,
    refill_period: Duration,
    state: Mutex<State>,
}

impl TokenBucket {
    /// Create a full bucket with the given refill rate.
    ///
    /// refill_tokens tokens are added over each refill_period, up to
    /// capacity. Zero values are rejected rather than silently clamped.
    pub fn new(
        capacity: u32,
        refill_tokens: u32,
        refill_period: Duration,
    ) -> Result<Self, ConfigError> {
        if capacity == 0 {
            return Err(ConfigError::ZeroCapacity);
        }
        if refill_tokens == 0 {
            return Err(ConfigError::ZeroRefillTokens);
        }
        if refill_period.is_zero() {
            return Err(ConfigError::ZeroRefillPeriod);
        }
        Ok(Self {
            capacity: f64::from(capacity),
            refill_tokens: f64::from(refill_tokens),
            refill_period,
            state: Mutex::new(State {
                tokens: f64::from(capacity),
                last_refill: Instant::now(),
            }),
        })
    }

    /// Attempt to consume one token using the current monotonic clock.
    pub fn try_acquire(&self) -> bool {
        self.try_acquire_at(Instant::now())
    }

    /// Attempt to consume one token at a supplied monotonic instant.
    ///
    /// Supplying an earlier instant never moves the stored clock backwards.
    /// The mutex covers both refill and consumption so concurrent callers
    /// cannot exceed the configured allowance.
    pub fn try_acquire_at(&self, now: Instant) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if now > state.last_refill {
            let elapsed = now.duration_since(state.last_refill).as_secs_f64();
            let period = self.refill_period.as_secs_f64();
            let earned = elapsed / period * self.refill_tokens;
            state.tokens = (state.tokens + earned).min(self.capacity);
            state.last_refill = now;
        }
        if state.tokens < 1.0 {
            return false;
        }
        state.tokens -= 1.0;
        true
    }

    /// Return the current available token count after applying refill.
    pub fn available_at(&self, now: Instant) -> f64 {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if now > state.last_refill {
            let elapsed = now.duration_since(state.last_refill).as_secs_f64();
            let earned = elapsed / self.refill_period.as_secs_f64() * self.refill_tokens;
            state.tokens = (state.tokens + earned).min(self.capacity);
            state.last_refill = now;
        }
        state.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn rejects_zero_configuration() {
        assert_eq!(TokenBucket::new(0, 1, Duration::from_secs(1)).err(),
                   Some(ConfigError::ZeroCapacity));
        assert_eq!(TokenBucket::new(1, 0, Duration::from_secs(1)).err(),
                   Some(ConfigError::ZeroRefillTokens));
        assert_eq!(TokenBucket::new(1, 1, Duration::ZERO).err(),
                   Some(ConfigError::ZeroRefillPeriod));
    }

    #[test]
    fn starts_full_and_rejection_does_not_consume_tokens() {
        let bucket = TokenBucket::new(2, 1, Duration::from_secs(1)).unwrap();
        let now = Instant::now();
        assert!(bucket.try_acquire_at(now));
        assert!(bucket.try_acquire_at(now));
        assert!(!bucket.try_acquire_at(now));
        assert!(!bucket.try_acquire_at(now));
    }

    #[test]
    fn refills_at_exact_period_boundary() {
        let bucket = TokenBucket::new(2, 1, Duration::from_secs(1)).unwrap();
        let now = Instant::now();
        assert!(bucket.try_acquire_at(now));
        assert!(bucket.try_acquire_at(now));
        assert!(!bucket.try_acquire_at(now));
        assert!(bucket.try_acquire_at(now + Duration::from_secs(1)));
    }

    #[test]
    fn preserves_fractional_refill_and_caps_capacity() {
        let bucket = TokenBucket::new(2, 1, Duration::from_secs(2)).unwrap();
        let now = Instant::now();
        bucket.try_acquire_at(now);
        bucket.try_acquire_at(now);
        assert!(!bucket.try_acquire_at(now));
        assert!((bucket.available_at(now + Duration::from_secs(1)) - 0.5).abs() < 0.0001);
        assert!((bucket.available_at(now + Duration::from_secs(20)) - 2.0).abs() < 0.0001);
    }

    #[test]
    fn earlier_timestamp_does_not_rewind_refill_clock() {
        let bucket = TokenBucket::new(2, 1, Duration::from_secs(1)).unwrap();
        let now = Instant::now();
        bucket.try_acquire_at(now);
        bucket.try_acquire_at(now);
        assert!(!bucket.try_acquire_at(now));
        assert!(bucket.try_acquire_at(now + Duration::from_secs(1)));
        assert!(!bucket.try_acquire_at(now));
    }

    #[test]
    fn concurrent_callers_cannot_overspend_initial_capacity() {
        let bucket = Arc::new(TokenBucket::new(50, 1, Duration::from_secs(3600)).unwrap());
        let threads: Vec<_> = (0..16)
            .map(|_| {
                let bucket = Arc::clone(&bucket);
                thread::spawn(move || {
                    (0..100).filter(|_| bucket.try_acquire()).count()
                })
            })
            .collect();
        let accepted: usize = threads.into_iter().map(|t| t.join().unwrap()).sum();
        assert_eq!(accepted, 50);
    }
}
