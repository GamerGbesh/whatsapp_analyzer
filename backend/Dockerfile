FROM rust:1.88-bookworm AS builder
WORKDIR /app

# Cache dependencies first.
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release --bin main

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/main /usr/local/bin/whatsapp-analyzer

EXPOSE 3000
ENV RUST_LOG=info
CMD ["/usr/local/bin/whatsapp-analyzer"]
