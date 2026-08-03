# Stage 1: Build the release binary
FROM rust:1.80-slim AS builder

WORKDIR /usr/src/inflection-rs
COPY . .

RUN cargo build --release --bin inflection

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/inflection-rs/target/release/inflection /usr/local/bin/inflection

ENTRYPOINT ["/usr/local/bin/inflection"]
CMD ["--help"]
