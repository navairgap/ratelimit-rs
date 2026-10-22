# ratelimit-rs

A planned rate-limiting library for Rust services.

> **Status: design stage.** This repository currently contains documentation only; there is no published crate API or implementation yet.

## Goal

Provide small, predictable rate-limit primitives that can be embedded in a service and tested independently of a web framework.

## Initial design

The first milestone should implement one in-memory algorithm before adding middleware or distributed storage. The API should make its time source injectable for deterministic tests and should document whether a rejected request consumes tokens.

Potential algorithms, to be evaluated after the first implementation:

- **Token bucket:** allows bounded bursts while limiting the long-term rate.
- **Fixed window:** simple and inexpensive, but permits bursts around window boundaries.
- **Sliding window:** smoother enforcement with additional state.

## Requirements for correctness

- Define behavior at exact refill and window boundaries.
- Avoid panics for invalid configuration; return a clear error.
- Use monotonic time for elapsed-time calculations.
- Bound per-key state or document the eviction policy.
- Explain that in-memory limits are per process and do not coordinate across replicas.
- Add concurrency tests before advertising thread safety.

## Planned milestones

- [ ] Write down API semantics and configuration validation.
- [ ] Implement a single-threaded token bucket with unit tests.
- [ ] Add concurrency tests and benchmarks with reproducible commands.
- [ ] Consider a Tower integration after the core API stabilizes.
- [ ] Evaluate a distributed backend separately; do not imply cross-process guarantees for the in-memory mode.

## Benchmarks

No performance claims are made yet. Published numbers should include hardware, compiler version, workload, and reproducible commands.

## License

MIT. See [LICENSE](LICENSE).


## Behavioral guarantees

Use monotonic time and an injectable clock for deterministic tests. Document capacity-boundary behavior, token consumption on rejection, key cleanup, and invalid configuration. Exercise concurrent access so calls cannot exceed the configured allowance.
