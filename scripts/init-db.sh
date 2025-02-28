#!/bin/bash
set -e

# Wait for PostgreSQL to be ready
until PGPASSWORD=postgres psql -h "postgres" -U "postgres" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing migrations"

# Run migrations
sqlx database create
sqlx migrate run

exec "$@" 