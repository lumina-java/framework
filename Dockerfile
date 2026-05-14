# Stage 1: Builder
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

# Copy only Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY lumina_macros ./lumina_macros

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    mkdir -p src/bin && echo "fn main() {}" > src/bin/lumina.rs && \
    cargo build --release && \
    rm -rf src

# Copy the real source code
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=builder /usr/src/app/target/release/lumina-server /app/lumina-server
COPY --from=builder /usr/src/app/target/release/lumina /app/lumina

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
