-- Add migration script here
DROP TYPE IF EXISTS subscriber_status CASCADE; 
CREATE TYPE subscriber_status AS ENUM ('enabled', 'disabled', 'blocklisted');

DROP EXTENSION IF EXISTS "uuid-ossp"; 
CREATE EXTENSION "uuid-ossp";

-- subscribers
DROP TABLE IF EXISTS subscribers CASCADE;
CREATE TABLE subscribers (
    id              SERIAL PRIMARY KEY,
    uuid            uuid NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    email           TEXT NOT NULL UNIQUE,
    name            TEXT,
    attribs         JSONB NOT NULL DEFAULT '{}',
    status          subscriber_status NOT NULL DEFAULT 'enabled',
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

DROP INDEX IF EXISTS idx_subs_email; 
CREATE UNIQUE INDEX idx_subs_email ON subscribers(LOWER(email));

DROP INDEX IF EXISTS idx_subs_status; 
CREATE INDEX idx_subs_status ON subscribers(status);

DROP INDEX IF EXISTS idx_subs_id_status; 
CREATE INDEX idx_subs_id_status ON subscribers(id, status);

DROP INDEX IF EXISTS idx_subs_created_at; 
CREATE INDEX idx_subs_created_at ON subscribers(created_at);

DROP INDEX IF EXISTS idx_subs_updated_at; 
CREATE INDEX idx_subs_updated_at ON subscribers(updated_at);