-- Add migration script here
-- Add migration script here
-- Add migration script here
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


-- Add migration script here
-- Add migration script here
-- Add migration script here
DROP TABLE IF EXISTS campaigns CASCADE;
DROP TYPE IF EXISTS campaign_status CASCADE; CREATE TYPE campaign_status AS ENUM ('draft', 'running', 'scheduled', 'paused', 'cancelled', 'finished');
DROP TYPE IF EXISTS campaign_type CASCADE; CREATE TYPE campaign_type AS ENUM ('regular', 'optin');
DROP TYPE IF EXISTS content_type CASCADE; CREATE TYPE content_type AS ENUM ('richtext', 'html', 'plain', 'markdown');


DROP TABLE IF EXISTS campaigns CASCADE;
CREATE TABLE campaigns (
    id               SERIAL PRIMARY KEY,
    uuid uuid        NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    name             TEXT NOT NULL, 
    subject          TEXT NOT NULL,
    from_email       TEXT NOT NULL,
    body             TEXT NOT NULL,
    altbody          TEXT NULL,
    content_type     content_type NOT NULL DEFAULT 'richtext',
    send_at          TIMESTAMP WITH TIME ZONE,
    headers          JSONB NOT NULL DEFAULT '[]',
    status           campaign_status NOT NULL DEFAULT 'draft',
    tags             VARCHAR(100)[],

    -- The subscription statuses of subscribers to which a campaign will be sent.
    -- For opt-in campaigns, this will be 'unsubscribed'.
    campaign_type campaign_type DEFAULT 'regular',

    -- The ID of the messenger backend used to send this campaign. 
    messenger        TEXT NOT NULL,
    template_id      INTEGER REFERENCES templates(id) ON DELETE SET DEFAULT DEFAULT 1,

    -- Progress and stats.
    to_send            INT NOT NULL DEFAULT 0,
    sent               INT NOT NULL DEFAULT 0,
    max_subscriber_id  INT NOT NULL DEFAULT 0,
    last_subscriber_id INT NOT NULL DEFAULT 0,

    -- Publishing.
    archive             BOOLEAN NOT NULL DEFAULT false,
    archive_slug        TEXT NULL UNIQUE,
    archive_template_id INTEGER REFERENCES templates(id) ON DELETE SET DEFAULT DEFAULT 1,
    archive_meta        JSONB NOT NULL DEFAULT '{}',

    started_at       TIMESTAMP WITH TIME ZONE,
    created_at       TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at       TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
DROP INDEX IF EXISTS idx_camps_status; CREATE INDEX idx_camps_status ON campaigns(status);
DROP INDEX IF EXISTS idx_camps_name; CREATE INDEX idx_camps_name ON campaigns(name);
DROP INDEX IF EXISTS idx_camps_created_at; CREATE INDEX idx_camps_created_at ON campaigns(created_at);
DROP INDEX IF EXISTS idx_camps_updated_at; CREATE INDEX idx_camps_updated_at ON campaigns(updated_at);



DROP TABLE IF EXISTS campaign_lists CASCADE;
CREATE TABLE campaign_lists (
    id           BIGSERIAL PRIMARY KEY,
    campaign_id  INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE ON UPDATE CASCADE,

    -- Lists may be deleted, so list_id is nullable
    -- and a copy of the original list name is maintained here.
    list_id      INTEGER NULL REFERENCES lists(id) ON DELETE SET NULL ON UPDATE CASCADE,
    list_name    TEXT NOT NULL DEFAULT ''
);
CREATE UNIQUE INDEX ON campaign_lists (campaign_id, list_id);
DROP INDEX IF EXISTS idx_camp_lists_camp_id; CREATE INDEX idx_camp_lists_camp_id ON campaign_lists(campaign_id);
DROP INDEX IF EXISTS idx_camp_lists_list_id; CREATE INDEX idx_camp_lists_list_id ON campaign_lists(list_id);

DROP TABLE IF EXISTS sequence_emails CASCADE;
CREATE TABLE sequence_emails (

        id SERIAL PRIMARY KEY,
        campaign_id INTEGER REFERENCES campaigns(id) ON DELETE CASCADE,
        position INTEGER NOT NULL,
        
        --Contenu de l'email--
        subject TEXT NOT NULL,
        body TEXT NOT NULL,
        template_id INTEGER REFERENCES templates(id),
        content_type content_type NOT NULL DEFAULT 'richtext',
        
        -- Heure de l'envoie --
        
        send_at TIMESTAMP WITH TIME ZONE,
        
        --Métadonnés --
        
        metadata JSONB DEFAULT '{}', 
        is_active BOOLEAN DEFAULT true,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        
);

-- Index pour optimiser les requêtes
CREATE INDEX idx_sequence_emails_campaign ON sequence_emails(campaign_id);
CREATE INDEX idx_sequence_emails_position ON sequence_emails(campaign_id, position);
CREATE INDEX idx_sequence_emails_send_at ON sequence_emails(send_at);

-- Modifications de la table campaigns
ALTER TABLE campaigns 
    DROP COLUMN body,
    DROP COLUMN altbody,
    DROP COLUMN content_type,
    DROP COLUMN template_id,
    DROP COLUMN send_at,
    ADD COLUMN sequence_start_date TIMESTAMP WITH TIME ZONE,  -- Date de début de la séquence
    ADD COLUMN sequence_end_date TIMESTAMP WITH TIME ZONE;    -- Date de fin de la séquence


DROP TABLE IF EXISTS email_views CASCADE;
CREATE TABLE email_views (
        id SERIAL PRIMARY KEY,
        sequence_email_id INTEGER REFERENCES sequence_emails(id) ON DELETE CASCADE,
        subscriber_id INTEGER REFERENCES subscribers(id) ON DELETE CASCADE,
        campaign_id INTEGER REFERENCES campaigns(id) ON DELETE CASCADE,
        
        
        -- Informations sur l'ouverture
        opened_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
         ip_address TEXT,
         user_agent TEXT,    
         
             -- Informations de localisation
            country TEXT,
            city TEXT,
            region TEXT,
            latitude TEXT,
            longitude TEXT,
            
                metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index pour optimiser les requêtes
CREATE INDEX idx_email_views_sequence_email ON email_views(sequence_email_id);
CREATE INDEX idx_email_views_subscriber ON email_views(subscriber_id);
CREATE INDEX idx_email_views_opened_at ON email_views(opened_at);
CREATE INDEX idx_email_views_location ON email_views(country, city);


-- Ensure no duplicate entries for the same subscriber_id and sequence_email_id
ALTER TABLE email_views
ADD CONSTRAINT unique_subscriber_sequence UNIQUE (subscriber_id, sequence_email_id);