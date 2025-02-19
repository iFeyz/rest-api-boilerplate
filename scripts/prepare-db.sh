#!/bin/bash
set -e

# Ensure postgres is running
docker-compose up postgres -d

# Wait for postgres to be ready
until PGPASSWORD=postgres psql -h localhost -U postgres -d postgres -c '\q' 2>/dev/null; do
  echo "Waiting for postgres..."
  sleep 1
done

echo "Creating database..."
PGPASSWORD=postgres psql -h localhost -U postgres -c "DROP DATABASE IF EXISTS rust_api_db;"
PGPASSWORD=postgres psql -h localhost -U postgres -c "CREATE DATABASE rust_api_db;"

echo "Running migrations..."
cargo sqlx database create
cargo sqlx migrate run

echo "Preparing SQLx data..."
cargo sqlx prepare --workspace

echo "Database preparation complete!" 