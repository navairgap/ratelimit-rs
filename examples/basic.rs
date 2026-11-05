use ratelimit_rs::TokenBucket;
use std::time::Duration;

fn main() {
    // Allow a burst of 10 requests, then refill 2 tokens per second.
    let limiter = TokenBucket::new(10, 2, Duration::from_secs(1))
        .expect("valid limiter configuration");

    for request in 1..=12 {
        if limiter.try_acquire() {
            println!("request {request}: accepted");
        } else {
            println!("request {request}: rate limited");
        }
    }
}
