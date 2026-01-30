# Ratelimiter

Lightweight rate-limiting service written in Rust. It exposes an HTTP API that answers whether a request is allowed
based on a fixed-window algorithm and keeps state in memory.

This project is intended as a minimal, self-contained rate limiter for local development, experiments, and as a
foundation for adding more algorithms or storage backends.
