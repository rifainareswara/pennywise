# Build stage
FROM rust:slim AS builder
WORKDIR /app

# Install library pendukung untuk kompilasi
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Trick caching dependensi
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true

# Build aplikasi asli
COPY . .
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Sesuaikan nama binary dengan nama project di Cargo.toml
COPY --from=builder /app/target/release/pennywise-backend /usr/local/bin/app
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app
EXPOSE 3000

CMD ["app"]