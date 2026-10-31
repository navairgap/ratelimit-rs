# Clock and refill precision contract (proposed)

A rate limiter must measure elapsed time without depending on wall-clock adjustments. This contract describes the clock boundary for a future implementation.

## Clock source

- Use a monotonic clock for elapsed durations; never calculate refill from calendar timestamps.
- Make the clock injectable so tests can advance time without sleeping.
- Treat equal timestamps as zero elapsed time.
- Defensively handle a test clock that moves backward by treating the elapsed duration as zero. Production behavior should rely on a monotonic source.
- Keep wall-clock timestamps out of the bucket arithmetic. They may be included separately in diagnostics.

## Refill arithmetic

Represent rates and elapsed durations with a documented unit and precision. Avoid floating-point accumulation that can create or destroy tokens near a boundary. Preserve fractional refill using an integer remainder or a rational representation, and cap the accumulated token count at capacity.

Check multiplication and addition for overflow before updating the bucket. Large configured durations or elapsed intervals must not wrap into a larger allowance. Long idle periods may fill a bucket, but can never grant more than capacity.

## Retry timing

When fewer than one token is available, calculate the wait until one token can be consumed. Round a positive duration upward to the advertised unit so clients are not told to retry earlier than the limiter expects. Document whether the returned duration is an estimate when clock resolution is coarse.

## Deterministic tests

Use a fake monotonic clock to test zero elapsed time, an exact refill boundary, just-before and just-after the boundary, fractional accumulation across multiple checks, a long idle period, capacity saturation, backward test-clock movement, and arithmetic near the largest supported duration. Tests must not sleep or depend on machine wall-clock time.