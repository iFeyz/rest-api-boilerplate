-- Add migration script here
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
-- Add migration script here
DROP TYPE IF EXISTS list_type CASCADE; CREATE TYPE list_type AS ENUM ('public', 'private', 'temporary');
DROP TYPE IF EXISTS list_optin CASCADE; CREATE TYPE list_optin AS ENUM ('single', 'double');

-- lists
DROP TABLE IF EXISTS lists CASCADE;
CREATE TABLE lists (
    id              SERIAL PRIMARY KEY,
    uuid            uuid NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    name            TEXT NOT NULL,
    type            list_type NOT NULL DEFAULT 'public',
    optin           list_optin NOT NULL DEFAULT 'single',
    tags            VARCHAR(100)[],
    description     TEXT NOT NULL DEFAULT '',

    created_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
DROP INDEX IF EXISTS idx_lists_type; CREATE INDEX idx_lists_type ON lists(type);
DROP INDEX IF EXISTS idx_lists_optin; CREATE INDEX idx_lists_optin ON lists(optin);
DROP INDEX IF EXISTS idx_lists_name; CREATE INDEX idx_lists_name ON lists(name);
DROP INDEX IF EXISTS idx_lists_created_at; CREATE INDEX idx_lists_created_at ON lists(created_at);
DROP INDEX IF EXISTS idx_lists_updated_at; CREATE INDEX idx_lists_updated_at ON lists(updated_at);

-- Add migration script here
DROP TYPE IF EXISTS template_type CASCADE; CREATE TYPE template_type AS ENUM ('campaign', 'tx');

-- templates
DROP TABLE IF EXISTS templates CASCADE;
CREATE TABLE templates (
    id              SERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    type            template_type NOT NULL DEFAULT 'campaign',
    subject         TEXT NOT NULL,
    body            TEXT NOT NULL,
    is_default      BOOLEAN NOT NULL DEFAULT false,

    created_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
CREATE UNIQUE INDEX ON templates (is_default) WHERE is_default = true;

-- Add migration script here
DROP TYPE IF EXISTS subscription_status CASCADE; CREATE TYPE subscription_status AS ENUM ('unconfirmed', 'confirmed', 'unsubscribed');

DROP TABLE IF EXISTS subscriber_lists CASCADE;
CREATE TABLE subscriber_lists (
    subscriber_id      INTEGER REFERENCES subscribers(id) ON DELETE CASCADE ON UPDATE CASCADE,
    list_id            INTEGER NULL REFERENCES lists(id) ON DELETE CASCADE ON UPDATE CASCADE,
    meta               JSONB NOT NULL DEFAULT '{}',
    status             subscription_status NOT NULL DEFAULT 'unconfirmed',

    created_at         TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at         TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    PRIMARY KEY(subscriber_id, list_id)
);
DROP INDEX IF EXISTS idx_sub_lists_sub_id; CREATE INDEX idx_sub_lists_sub_id ON subscriber_lists(subscriber_id);
DROP INDEX IF EXISTS idx_sub_lists_list_id; CREATE INDEX idx_sub_lists_list_id ON subscriber_lists(list_id);
DROP INDEX IF EXISTS idx_sub_lists_status; CREATE INDEX idx_sub_lists_status ON subscriber_lists(status);
