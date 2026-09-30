# ratelimit-rs

A rate-limiting middleware crate for Rust web services

> 🚧 **Status: planning** — architecture and README first, code next.

## Why

Every real API needs rate limiting, and Rust's ecosystem deserves a clean, composable crate. Sliding window, token bucket, and fixed window — as tower/axum middleware with Redis or in-memory backends.

## Planned features

- Token bucket, sliding window, fixed window algorithms
- In-memory and Redis-backed stores
- axum/tower middleware + a plain function API
- Per-key limits (IP, user, API key) with custom quotas

## Stack

`rust` `tokio` `redis` `axum`

## Notes

Start with in-memory token bucket; Redis backend second. Property-based tests from day one.

## License

MIT, see [LICENSE](LICENSE).

---
maintained · verified 2026-09-30
