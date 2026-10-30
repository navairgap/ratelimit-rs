# algorithm conformance

Use an injectable clock so tests do not sleep. Cover capacity boundaries, refill over elapsed monotonic time, backward clock movement, invalid configuration, large time jumps, rejected-request token behavior, concurrent calls, and bounded state cleanup. Reject zero or negative capacity and refill rates. Every algorithm should pass a shared conformance suite before becoming public API.

## Refill arithmetic boundaries

Test refill calculations at capacity, one unit below capacity, and durations large enough to approach the integer representation limit. Implementations should saturate at capacity without overflowing intermediate multiplication, and rejected requests must not consume tokens unless the contract explicitly says otherwise. Use a deterministic fake monotonic clock and assert exact token counts at the instant before and after a refill boundary; avoid sleep-based tests.
