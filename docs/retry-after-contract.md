# Retry hint and HTTP Retry-After semantics (proposed)

ratelimit-rs is in the design stage. This document defines how a future core
decision can expose a retry hint without promising more precision than its
clock and algorithm provide.

## Core decision

A denied decision may include the duration until one request is expected to
become admissible. The duration is an estimate derived from the limiter's
monotonic clock and configured refill policy.

- An allowed decision does not need a retry hint.
- A denied request does not consume an additional token unless the algorithm's
  documented contract explicitly states otherwise.
- A retry hint must be non-negative and must not overflow when converted to
  the public duration type.
- When the wait is positive but smaller than the output unit, round upward.
  Never tell a caller to retry earlier than the next possible admission time.
- If the limiter cannot calculate a trustworthy hint, return no hint rather
  than inventing a duration.

## HTTP adapter mapping

An optional HTTP adapter may map a denied decision to the standard
`Retry-After` delay-seconds form:

- Round a positive wait up to the next whole second.
- A positive sub-second wait is represented as `1`, not `0`.
- Do not emit a negative, fractional, NaN, or overflowing value.
- Do not emit `Retry-After` for an allowed request.
- Keep HTTP formatting outside the core algorithm so the library remains
  independent of a web framework.

The delay-seconds form is preferred over an HTTP-date for this mapping because
the core limiter measures elapsed time with a monotonic clock, not a calendar
clock. An adapter must not convert a monotonic instant into a guessed wall-clock
timestamp.

## Scope and honesty

A retry hint is local to the limiter instance that produced it. An in-memory
limiter does not coordinate across processes or replicas, so another instance
may admit or reject the same caller differently. Do not describe a local hint
as a globally reserved slot or a guarantee that a retry will succeed.

If the key is evicted or the process restarts, the next request may see a
different bucket state. This behavior must be documented by any adapter that
exposes retry hints to clients.

## Deterministic tests

Use an injected fake monotonic clock. Cover:

- no hint for an allowed request;
- a positive sub-second wait rounds to one second in HTTP output;
- an exact whole-second wait remains unchanged;
- a fractional duration never rounds down;
- zero, negative, and overflowing conversion inputs are rejected or omitted;
- a denied request leaves the remaining token count unchanged;
- a clock adjustment cannot increase the retry allowance;
- per-process scope is reflected in API documentation.

Do not use sleep-based tests or make distributed consistency claims for the
in-memory design.
