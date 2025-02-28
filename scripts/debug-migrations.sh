#!/bin/bash
set -e

echo "Debugging database migrations..."

# Extraire les informations de connexion
DB_HOST=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@([^:]+):.*/\1/')
DB_PORT=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@[^:]+:([0-9]+).*/\1/')
DB_USER=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/([^:]+):.*/\1/')
DB_PASSWORD=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:([^@]+)@.*/\1/')
DB_NAME=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@[^:]+:[0-9]+\/([^?]+).*/\1/')

echo "Connection details:"
echo "Host: $DB_HOST"
echo "Port: $DB_PORT"
echo "User: $DB_USER"
echo "Database: $DB_NAME"

echo "Testing database connection..."
if PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c '\conninfo'; then
  echo "Connection successful!"
else
  echo "Connection failed!"
  exit 1
fi

echo "Listing migration files:"
find /app/migrations -type f | sort

echo "Checking migration status:"
sqlx migrate info

echo "Debug complete." 