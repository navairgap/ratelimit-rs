# Concurrency verification plan (proposed)

Thread-safety must be an explicit API guarantee, not an assumption inferred from a type being shareable. This plan describes the tests required before the library advertises concurrent use.

## Contract to decide first

Document whether a limiter instance is single-threaded or safe to share between threads. If it is shareable, define the synchronization boundary for a bucket lookup, refill calculation, and token consumption. The check-and-consume operation must be atomic with respect to other callers for the same key.

Document whether calls for different keys contend on one global lock or can proceed independently. Do not promise lock-free behavior or a particular throughput without measurements.

## Required scenarios

- Run many concurrent requests against one initially full bucket and verify no more than capacity requests are accepted before refill.
- Advance a fake clock and verify that the aggregate accepted count matches the configured refill allowance.
- Run independent keys concurrently and verify that one key cannot consume another key's tokens.
- Race key creation with the configured state cap and verify that the cap is never exceeded.
- Exercise inactive-key expiration while requests are arriving for active keys.
- Verify that rejected requests do not consume tokens or reset another caller's refill state.

## Test design

Use barriers to coordinate worker start, a deterministic clock where possible, and bounded worker completion. Avoid tests whose correctness depends on scheduler timing or arbitrary sleeps. Record the accepted/rejected counts and assert the limiter's documented invariant, rather than asserting a particular thread wins a race.

## Release gate

Run the concurrency suite under the supported Rust toolchain and include a stress-test recipe that can be repeated locally. A passing finite test does not prove correctness under every schedule; document the scope of the guarantee and retain the invariant-focused tests as regression coverage.