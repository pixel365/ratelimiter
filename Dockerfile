FROM rust:1.93.0-alpine3.23@sha256:69d7b9d9aeaf108a1419d9a7fcf7860dcc043e9dbd1ab7ce88e44228774d99e9 AS builder

RUN apk add --no-cache ca-certificates musl-dev \
    && addgroup -S ratelimiter \
    && adduser --uid 19998 --shell /bin/false -S ratelimiter -G ratelimiter \
    && cat /etc/passwd | grep ratelimiter > /etc/passwd_ratelimiter \
    && cat /etc/group  | grep ratelimiter > /etc/group_ratelimiter

WORKDIR /app

COPY Cargo.* ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY ./src ./src

RUN cargo build --release
RUN ldd /app/target/release/ratelimiter || true

FROM scratch

WORKDIR /app

COPY --from=builder /etc/passwd_ratelimiter /etc/passwd
COPY --from=builder /etc/group_ratelimiter  /etc/group
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder --chown=ratelimiter:ratelimiter /app/target/release/ratelimiter /ratelimiter

USER ratelimiter

ENTRYPOINT ["/ratelimiter", "--http-host", "0.0.0.0", "--http-port", "3000"]
