# Stage 1: Build Frontend (Astro)
FROM oven/bun:1 AS frontend-builder
WORKDIR /app
COPY web/package.json web/bun.lock ./web/
WORKDIR /app/web
RUN bun install --frozen-lockfile
COPY web/ .
RUN bun run build

# Stage 2: Build Backend (Rust)
FROM rust:1-slim-bookworm AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3
# Create a blank Rust project to cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Copy built frontend assets so rust-embed can find them
COPY --from=frontend-builder /app/web/dist ./web/dist

# Initialize a dummy DB for sqlx compile-time verification
ENV DATABASE_URL=sqlite:alembic.db
RUN sqlite3 alembic.db < crates/alembic-core/src/schema.sql

# Build the binary
RUN cargo build --release -p alembic-server

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates sqlite3 libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /app/target/release/alembic-server .

# Default env vars
ENV DATABASE_URL=sqlite:/data/alembic.db
ENV RUST_LOG=info

# Create data directory
RUN mkdir -p /data
VOLUME /data

# Expose port
EXPOSE 3000

CMD ["./alembic-server"]
