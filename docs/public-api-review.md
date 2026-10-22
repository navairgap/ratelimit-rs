# public api review

Review public APIs for clear units, bounded state, and explicit behavior at capacity boundaries. Prefer monotonic time and an injectable clock. Document whether rejected requests consume tokens, whether keys are case-sensitive, and how stale keys are removed. Avoid panics for invalid user configuration; return structured errors. Keep algorithm-specific behavior behind tests shared across implementations.
