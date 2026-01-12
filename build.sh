#!/usr/bin/env bash
set -euo pipefail

echo "Building Alembic"

# build frontend
cd web
echo "Installing frontend deps"
bun install

echo "Building frontend"
bun run build
echo "Frontend built"

# build backend
cd ..
echo "Building backend"
cargo build --release
echo "Backend built"

# run server
cd target/release
chmod +x ./alembic-server
exec ./alembic-server