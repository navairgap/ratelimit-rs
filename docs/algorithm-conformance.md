# algorithm conformance

Use an injectable clock so tests do not sleep. Cover capacity boundaries, refill over elapsed monotonic time, backward clock movement, invalid configuration, large time jumps, rejected-request token behavior, concurrent calls, and bounded state cleanup. Reject zero or negative capacity and refill rates. Every algorithm should pass a shared conformance suite before becoming public API.
