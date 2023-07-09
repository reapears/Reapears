-- Setup UP migration
-- Reapears database defenition

-- Schemas
CREATE SCHEMA IF NOT EXISTS services;
CREATE SCHEMA IF NOT EXISTS features;
CREATE SCHEMA IF NOT EXISTS archives;
CREATE SCHEMA IF NOT EXISTS accounts;
CREATE SCHEMA IF NOT EXISTS auth;


-- Accounts(Schema) Tables

-- Users table
DROP TABLE IF EXISTS accounts.users;
CREATE TABLE accounts.users(
    id uuid PRIMARY KEY,
    first_name text NOT NULL,
    last_name text,
    gender text,
    date_of_birth date,
    phc_string text NOT NULL,
    is_farmer boolean NOT NULL,
    is_staff boolean NOT NULL,
    is_superuser boolean NOT NULL,
    last_login timestamptz,
    date_joined timestamptz NOT NULL,
    identity_verified boolean NOT NULL DEFAULT FALSE,
    account_locked boolean NOT NULL,
    account_locked_reason text,
    account_locked_until date
);

-- Users profile table
DROP TABLE IF EXISTS accounts.user_profiles;
CREATE TABLE accounts.user_profiles(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    photo text UNIQUE,
    about text NOT NULL DEFAULT '',
    lives_at text,
    PRIMARY KEY(user_id)
);


-- User emails table
DROP TABLE IF EXISTS accounts.emails;
CREATE TABLE accounts.emails(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    email text UNIQUE NOT NULL,
    verified boolean NOT NULL,
    token bytea UNIQUE,
    token_generated_at timestamptz,
    PRIMARY KEY(user_id)
);


-- User phones table
DROP TABLE IF EXISTS accounts.phones;
CREATE TABLE accounts.phones(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    phone text UNIQUE NOT NULL,
    verified boolean NOT NULL,
    token text,
    token_generated_at timestamptz,
    PRIMARY KEY(user_id)
);

-- DROP TABLE IF EXISTS accounts.follows;
-- CREATE TABLE accounts.follows(
--     user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
--     follows_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
--     PRIMARY KEY(user_id, follows_id)
-- );


-- Temporary stores the new email the user want to change to
DROP TABLE IF EXISTS accounts.email_pending_updates;
CREATE TABLE accounts.email_pending_updates(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE NOT NULL UNIQUE,
    new_email text NOT NULL,
    previous_email_approval_code bytea UNIQUE NOT NULL,
    -- token will be set once the updates approved so it can be null
    new_email_verify_token bytea UNIQUE,
    email_change_approved boolean NOT NULL,
    generated_at timestamptz NOT NULL
);


-- Auth(Schema) Tables

-- User sessions table
DROP TABLE IF EXISTS auth.sessions;
CREATE TABLE auth.sessions(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    user_agent text NOT NULL,
    token bytea UNIQUE NOT NULL,
    created_at timestamptz NOT NULL,
    last_used_at timestamptz NOT NULL
);

-- Stores password reset token, send to the user email 
-- on password forgot
DROP TABLE IF EXISTS auth.password_reset_tokens;
CREATE TABLE auth.password_reset_tokens(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    token bytea UNIQUE NOT NULL, -- token hash
    token_generated_at timestamptz NOT NULL,
    PRIMARY KEY(user_id)
);


-- Stores api keys for authenticating frontend apps or users
DROP TABLE IF EXISTS auth.api_tokens;
CREATE TABLE auth.api_tokens(
    id SERIAL PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    token bytea NOT NULL UNIQUE,
    belongs_to text, -- app / user
    created_at timestamptz NOT NULL,
    last_used_at timestamptz NOT NULL,
    revoked boolean NOT NULL
);

-- Services(Schema) Tables

-- DROP TABLE IF EXISTS services.tags;
-- CREATE TABLE services.tags(
--     id uuid PRIMARY KEY,
--     label_id uuid REFERENCES models.content_types (id) ON DELETE CASCADE NOT NULL,
--     name text NOT NULL UNIQUE
-- );


DROP TABLE IF EXISTS services.cultivar_categories;
CREATE TABLE services.cultivar_categories(
    id uuid PRIMARY KEY,
    name text NOT NULL UNIQUE
);


DROP TABLE IF EXISTS services.cultivars;
CREATE TABLE services.cultivars(
    id uuid PRIMARY KEY,
    category_id uuid REFERENCES services.cultivar_categories (id),
    name text NOT NULL UNIQUE,
    image text
);


-- DROP TABLE IF EXISTS services.cultivar_tags;
-- CREATE TABLE services.cultivar_tags(
--     cultivar_id uuid REFERENCES services.cultivars (id) ON DELETE CASCADE,
--     tag_id uuid REFERENCES services.tags (id) ON DELETE CASCADE,
--     PRIMARY KEY(cultivar_id, tag_id)
-- );


DROP TABLE IF EXISTS services.farms;
CREATE TABLE services.farms(
    id uuid PRIMARY KEY,
    -- Owner id can be null because on user delete
    -- it will be archived and the owner_id will be set to null
    -- if it has archived harvests
    owner_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    name text NOT NULL,
    logo text,
    contact_number text,
    contact_email text,
    founded_at date,
    verified boolean NOT NULL DEFAULT FALSE,
    registered_on date NOT NULL,
    deleted boolean NOT NULL,
    deleted_at date
);


DROP TABLE IF EXISTS services.countries;
CREATE TABLE services.countries(
    id uuid PRIMARY KEY,
    name text NOT NULL UNIQUE
);


DROP TABLE IF EXISTS services.regions;
CREATE TABLE services.regions(
    id uuid PRIMARY KEY,
    country_id uuid REFERENCES services.countries (id) ON DELETE CASCADE,
    name text NOT NULL,
    UNIQUE(country_id, name)
);


-- Farm's locations table
DROP TABLE IF EXISTS services.locations;
CREATE TABLE services.locations(
    id uuid PRIMARY KEY,
    farm_id uuid REFERENCES services.farms (id) ON DELETE CASCADE NOT NULL,
    place_name text NOT NULL,
    region_id uuid REFERENCES services.regions (id),
    country_id uuid REFERENCES services.countries (id) NOT NULL,
    description text,
    coords jsonb,
    created_at date NOT NULL,
    deleted boolean NOT NULL,
    deleted_at date
);


-- Farm's rating table
DROP TABLE IF EXISTS services.farm_ratings;
CREATE TABLE services.farm_ratings(
    id uuid PRIMARY KEY,
    author_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    farm_id uuid REFERENCES services.farms (id) ON DELETE CASCADE,
    grade integer NOT NULL CHECK (grade > 0 AND grade <= 5), -- grade must be either 1, 2, 3, 4, or 5.
    comment text,
    updated_at timestamptz,
    created_at timestamptz NOT NULL
);


-- DROP TABLE IF EXISTS services.farm_tags;
-- CREATE TABLE services.farm_tags(
--     farm_id uuid REFERENCES services.farms (id) ON DELETE CASCADE,
--     tag_id uuid REFERENCES services.tags (id) ON DELETE CASCADE,
--     PRIMARY KEY(farm_id, tag_id)
-- );


-- DROP TABLE IF EXISTS services.location_tags;
-- CREATE TABLE services.location_tags(
--     location_id uuid REFERENCES services.locations (id) ON DELETE CASCADE,
--     tag_id uuid REFERENCES services.tags (id) ON DELETE CASCADE,
--     PRIMARY KEY(location_id, tag_id)
-- );

-- Harvest listings table
DROP TABLE IF EXISTS services.harvests;
CREATE TABLE services.harvests(
    id uuid PRIMARY KEY,
    cultivar_id uuid REFERENCES services.cultivars (id) NOT NULL,
    location_id uuid REFERENCES services.locations (id) NOT NULL,
    price jsonb NOT NULL,
    type text,
    description text,
    available_at date NOT NULL,
    images text[],
    updated_at timestamptz,
    finished boolean NOT NULL,
    finished_at date,
    created_at timestamptz NOT NULL,
    UNIQUE(cultivar_id, location_id, available_at)
);


-- -- User harvests `wishlists`
-- DROP TABLE IF EXISTS services.harvests_wishlist;
-- CREATE TABLE services.harvests_wishlist(
--     user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
--     harvest_id uuid REFERENCES services.harvests (id) ON DELETE CASCADE,
--     created_at timestamptz NOT NULL,
--     PRIMARY KEY(user_id, harvest_id)
-- );


-- -- Direct Messages(Shema) tables ??
DROP TABLE IF EXISTS features.direct_messages;
CREATE TABLE features.direct_messages(
    id uuid PRIMARY KEY,
    sender_id uuid REFERENCES accounts.users (id),
    receiver_id uuid REFERENCES accounts.users (id),
    content text NOT NULL,
    sent_at timestamptz NOT NULL
);


DROP TABLE IF EXISTS features.message_status;
CREATE TABLE features.message_status(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id),
    message_id uuid REFERENCES features.direct_messages (id),
    is_author boolean,
    is_read boolean NOT NULL,
    is_deleted boolean NOT NULL,
    read_at timestamptz,
    deleted_at timestamptz,
    UNIQUE(user_id, message_id)
);


-- VIEWS

DROP VIEW IF EXISTS services.active_locations;
CREATE VIEW services.active_locations AS (
	SELECT *
	FROM services.locations location_
	WHERE location_.deleted = false
);

DROP VIEW IF EXISTS services.active_farms;
CREATE VIEW services.active_farms AS (
	SELECT *
	FROM services.farms farm
	WHERE farm.deleted = false
    	AND farm.owner_id IS NOT NULL
);

DROP VIEW IF EXISTS services.active_harvests;
CREATE VIEW services.active_harvests AS (
	SELECT *
	FROM services.harvests harvest
	WHERE harvest.finished = false
);

