# ─── Stage 1: Builder ─────────────────────────────────────────────────────────
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

COPY . .

# Build dengan Docker BuildKit Cache Mounts untuk kompilasi super cepat
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release && \
    cp /usr/src/app/target/release/lumina /app-lumina && \
    (cp /usr/src/app/target/release/lumina-server /app-server 2>/dev/null || cp /app-lumina /app-server)

# ─── Stage 2: Runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=builder /app-server /app/lumina-server
COPY --from=builder /app-lumina /app/lumina

# Copy necessary assets
COPY resources /app/resources
COPY database /app/database
COPY storage /app/storage
COPY .env.example /app/.env

# Default environment variables
ENV APP_ENV=production
ENV APP_PORT=8000

EXPOSE 8000

CMD ["./lumina-server"]
