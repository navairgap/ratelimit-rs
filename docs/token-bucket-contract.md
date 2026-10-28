# Token bucket contract (proposed)

This document pins down the behavior a first in-memory token bucket should implement before its public API is considered stable.

## Configuration

- `capacity` must be greater than zero.
- `refill_tokens` must be greater than zero and `refill_period` must be nonzero.
- Invalid configuration returns a typed error; it must not panic or silently clamp values.
- Elapsed time uses a monotonic clock. The clock should be injectable in tests.

## Time and retry hints

Define attempt timing explicitly: the first request has attempt index 1, and a configured maximum-attempt count includes that initial request. Compute retry hints with a ceiling to the next whole time unit so a positive wait is never rounded down to zero. Accumulate fractional refill with integer remainder or another documented precision-preserving representation; avoid per-check rounding that creates or destroys tokens. Validate arithmetic before multiplication so extreme durations cannot overflow into a permissive limit.

## Request semantics

1. Refill tokens according to elapsed monotonic time, capped at capacity.
2. If at least one token is available, consume one and allow the request.
3. Otherwise reject without consuming additional tokens.
4. A rejected request may report the duration until one token is expected, but must not promise an exact time if the caller uses a shared clock with coarse resolution.

Fractional refill should be accumulated rather than discarded on each check. A backward clock movement must not create tokens; monotonic time should make this impossible in production, while tests should cover defensive handling.

## Per-key lifecycle

If the API keeps a bucket per client key, document the maximum number of tracked keys and the expiration policy for inactive keys. Enforce the cap before inserting new state, and make eviction behavior deterministic enough to test. Never evict an active bucket silently in a way that grants a fresh burst to an already rate-limited key. Treat client keys as opaque identifiers and avoid logging raw tokens or credentials as keys.

## Concurrency and scope

The first implementation may be single-threaded if that limitation is explicit. If synchronization is added, document the lock scope and add concurrent stress tests. An in-memory bucket is per process and does not coordinate limits across replicas.

## Required tests

Cover empty and full buckets, exact refill boundaries, burst capacity, long idle periods, invalid configuration, and repeated rejection. Use a fake clock so tests are deterministic and fast. Do not publish throughput claims until a reproducible benchmark exists.
