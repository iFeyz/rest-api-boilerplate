#!/bin/bash
set -e

echo "Waiting for database..."
until PGPASSWORD=postgres psql -h "postgres" -U "postgres" -d "postgres" -c '\q' 2>/dev/null; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

echo "Creating database if it doesn't exist..."
PGPASSWORD=postgres psql -h "postgres" -U "postgres" -d "postgres" -c "CREATE DATABASE rust_api_db;" || true

echo "Database is up - running migrations"
cd /usr/local/bin
sqlx migrate run

echo "Starting application" 