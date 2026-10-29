# public api review

Review public APIs for clear units, bounded state, and explicit behavior at capacity boundaries. Prefer monotonic time and an injectable clock. Document whether rejected requests consume tokens, whether keys are case-sensitive, and how stale keys are removed. Avoid panics for invalid user configuration; return structured errors. Keep algorithm-specific behavior behind tests shared across implementations.

## Units and configuration boundaries

Use one documented unit for each public duration field and include the unit in names or types where practical. Reject negative durations and nonsensical zero values at construction time instead of letting each algorithm interpret them differently. Keep the chosen clock monotonic, and make retry-after values use a documented unit and rounding rule. Shared conformance tests should prove that each implementation follows the same contract.
