# Per-key state eviction contract (proposed)

An in-memory limiter must bound the number of keys it retains. Per-key buckets can otherwise turn attacker-controlled identifiers into unbounded memory growth.

## Required configuration

The implementation should expose and validate:

- a positive maximum number of tracked keys;
- an idle-expiration duration measured with the same monotonic clock used for refill;
- a documented behavior when the key cap is reached and no expired entry is eligible.

Do not invent a default that silently changes the rate-limit guarantee. The selected defaults and their memory/availability trade-offs must be documented before the API is considered stable.

## Eviction rules

1. Expire a key only after its configured idle interval has elapsed.
2. Do not evict an active key merely to admit a new key unless that behavior is an explicit, documented policy.
3. If no safe eviction candidate exists, reject creation of new state or return a typed capacity error.
4. Update last-seen time only according to a documented request policy; rejected requests must not keep arbitrary keys resident forever.
5. Make lookup, expiration, and insertion atomic when the limiter is shared across threads.

Treat keys as opaque values. Avoid logging raw API tokens, cookies, or other credentials if callers use them as key material.

## Deterministic tests

Use an injectable clock to cover an entry just before and just after idle expiry, exact-capacity insertion, attempted insertion when all entries are active, concurrent insertion at the cap, and repeated requests from one key. Assert that the configured state bound is never exceeded and that eviction cannot grant an undocumented fresh burst to a still-active key.
