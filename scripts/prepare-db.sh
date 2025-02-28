#!/bin/bash
set -e

# Ensure postgres is running
docker-compose up postgres -d

# Wait for postgres to be ready
until PGPASSWORD=postgres psql -h localhost -U postgres -d postgres -c '\q' 2>/dev/null; do
  echo "Waiting for postgres..."
  sleep 1
done

echo "Creating database if it doesn't exist..."
# Utiliser les variables extraites du DATABASE_URL
DB_HOST=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@([^:]+):.*/\1/')
DB_PORT=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@[^:]+:([0-9]+).*/\1/')
DB_USER=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/([^:]+):.*/\1/')
DB_PASSWORD=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:([^@]+)@.*/\1/')
DB_NAME=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@[^:]+:[0-9]+\/([^?]+).*/\1/')

PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -c "CREATE DATABASE $DB_NAME WITH OWNER $DB_USER;" || true

echo "Running migrations..."
sqlx migrate run

echo "Preparing SQLx data..."
cargo sqlx prepare --workspace

echo "Database preparation completed." 