#!/bin/bash
set -e

# Paramètres de connexion à partir des variables d'environnement
DB_HOST=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@([^:]+):.*/\1/')
DB_PORT=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@[^:]+:([0-9]+).*/\1/')
DB_USER=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/([^:]+):.*/\1/')
DB_PASSWORD=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:([^@]+)@.*/\1/')
DB_NAME=$(echo $DATABASE_URL | sed -E 's/^postgres:\/\/[^:]+:[^@]+@[^:]+:[0-9]+\/([^?]+).*/\1/')

echo "Waiting for PostgreSQL to be ready at $DB_HOST:$DB_PORT..."
until PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c '\q'; do
  echo "PostgreSQL is unavailable - sleeping"
  sleep 2
done

echo "PostgreSQL is up - executing migrations"
cd /app
sqlx migrate run

if [ $? -ne 0 ]; then
  echo "Migration failed! Exiting."
  exit 1
fi

echo "Migrations completed successfully - starting application"
exec "$@" 