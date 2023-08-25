-- Add up migration script here
-- Add up migration script here

-- Setup UP migration
-- Reapears database definition

-- ===== Schemas =====

CREATE SCHEMA IF NOT EXISTS services;
CREATE SCHEMA IF NOT EXISTS features;
CREATE SCHEMA IF NOT EXISTS archives;
CREATE SCHEMA IF NOT EXISTS accounts;
CREATE SCHEMA IF NOT EXISTS auth;


-- ===== Accounts(Schema) Tables =====

-- Users table
CREATE TABLE IF NOT EXISTS accounts.users(
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
CREATE TABLE IF NOT EXISTS accounts.user_profiles(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    photo text UNIQUE,
    about text NOT NULL DEFAULT '',
    lives_at text,
    PRIMARY KEY(user_id)
);


-- User emails table
CREATE TABLE IF NOT EXISTS accounts.emails(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    email text UNIQUE NOT NULL,
    verified boolean NOT NULL,
    token bytea UNIQUE,
    token_generated_at timestamptz,
    PRIMARY KEY(user_id)
);


-- User phones table
CREATE TABLE IF NOT EXISTS accounts.phones(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    phone text UNIQUE NOT NULL,
    verified boolean NOT NULL,
    token text,
    token_generated_at timestamptz,
    PRIMARY KEY(user_id)
);

-- DROP TABLE IF EXISTS accounts.follows;
-- CREATE TABLE IF NOT EXISTS accounts.follows(
--     user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
--     follows_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
--     PRIMARY KEY(user_id, follows_id)
-- );

-- Record users that requested their accounts to be deleted,
-- the user account will be deleted after 30 days the request issued.
CREATE TABLE IF NOT EXISTS accounts.account_delete_requests(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    requested_at timestamptz NOT NULL,
    PRIMARY KEY(user_id)
);

-- Temporary stores the new email the user want to change to
CREATE TABLE IF NOT EXISTS accounts.email_pending_updates(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE NOT NULL UNIQUE,
    new_email text NOT NULL,
    previous_email_approval_code bytea UNIQUE NOT NULL,
    -- token will be set once the updates approved so it can be null
    new_email_verify_token bytea UNIQUE,
    email_change_approved boolean NOT NULL,
    generated_at timestamptz NOT NULL
);


-- ===== Auth(Schema) Tables =====

-- User sessions table
CREATE TABLE IF NOT EXISTS auth.sessions(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    user_agent text NOT NULL,
    token bytea UNIQUE NOT NULL,
    created_at timestamptz NOT NULL,
    last_used_at timestamptz NOT NULL
);

-- Stores password reset token, send to the user email 
-- on password forgot
CREATE TABLE IF NOT EXISTS auth.password_reset_tokens(
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    token bytea UNIQUE NOT NULL, -- token hash
    token_generated_at timestamptz NOT NULL,
    PRIMARY KEY(user_id)
);


-- Stores api keys for authenticating frontend apps or users
CREATE TABLE IF NOT EXISTS auth.api_tokens(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    token bytea NOT NULL UNIQUE,
    belongs_to text NOT NULL, -- app / user
    created_at timestamptz NOT NULL,
    last_used_at timestamptz NOT NULL,
    revoked boolean NOT NULL
);

-- ===== Services(Schema) Tables  =====

CREATE TABLE IF NOT EXISTS services.cultivar_categories(
    id uuid PRIMARY KEY,
    name text NOT NULL UNIQUE
);


CREATE TABLE IF NOT EXISTS services.cultivars(
    id uuid PRIMARY KEY,
    category_id uuid REFERENCES services.cultivar_categories (id),
    name text NOT NULL UNIQUE,
    image text
);

CREATE TABLE IF NOT EXISTS services.farms(
    id uuid PRIMARY KEY,
    -- Owner id can be null since on user delete
    -- it will be archived and the owner_id will be set to null
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


CREATE TABLE IF NOT EXISTS services.countries(
    id uuid PRIMARY KEY,
    name text NOT NULL UNIQUE
);


CREATE TABLE IF NOT EXISTS services.regions(
    id uuid PRIMARY KEY,
    country_id uuid REFERENCES services.countries (id) ON DELETE CASCADE,
    name text NOT NULL,
    UNIQUE(country_id, name)
);


-- Farm's locations table
CREATE TABLE IF NOT EXISTS services.locations(
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
CREATE TABLE IF NOT EXISTS services.farm_ratings(
    id uuid PRIMARY KEY,
    author_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
    farm_id uuid REFERENCES services.farms (id) ON DELETE CASCADE,
    grade integer CHECK (grade > 0 AND grade <= 5), -- grade must be either 1, 2, 3, 4, or 5.
    comment text,
    -- used for when the rating is a reply.
    reply_to uuid REFERENCES services.farm_ratings (id),
    updated_at timestamptz,
    created_at timestamptz NOT NULL
);

-- Harvest listings table
CREATE TABLE IF NOT EXISTS services.harvests(
    id uuid PRIMARY KEY,
    cultivar_id uuid REFERENCES services.cultivars (id) NOT NULL,
    location_id uuid REFERENCES services.locations (id) NOT NULL,
    price jsonb NOT NULL,
    type text,
    description text,
    harvest_date date NOT NULL,
    images text[],
    updated_at timestamptz,
    finished boolean NOT NULL,
    finished_at date,
    created_at timestamptz NOT NULL,
    UNIQUE(cultivar_id, location_id, harvest_date, type)
);


-- -- User harvests `wishlists`
-- DROP TABLE IF EXISTS services.harvests_wishlist;
-- CREATE TABLE IF NOT EXISTS services.harvests_wishlist(
--     user_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE,
--     harvest_id uuid REFERENCES services.harvests (id) ON DELETE CASCADE,
--     created_at timestamptz NOT NULL,
--     PRIMARY KEY(user_id, harvest_id)
-- );


-- ===== Features-Schema =====

-- Harvest subscription
CREATE TABLE IF NOT EXISTS features.harvest_subscriptions(
    id uuid PRIMARY KEY,
    harvest_id uuid REFERENCES services.harvests (id) ON DELETE CASCADE NOT NULL UNIQUE,
    amount decimal NOT NULL,
    expires_at date NOT NULL,
    created_at timestamptz NOT NULL
);


-- Direct Messages tables
CREATE TABLE IF NOT EXISTS features.direct_messages(
    id uuid PRIMARY KEY,
    sender_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE NOT NULL,
    receiver_id uuid REFERENCES accounts.users (id) ON DELETE CASCADE NOT NULL,
    content text NOT NULL,
    sent_at timestamptz NOT NULL
);

-- Direct Messages status
CREATE TABLE IF NOT EXISTS features.message_status(
    message_id uuid REFERENCES features.direct_messages (id) ON DELETE CASCADE,
    is_read boolean NOT NULL,
    read_at timestamptz,
    sender_has_deleted boolean NOT NULL,
    receiver_has_deleted boolean NOT NULL,
    sender_deleted_at timestamptz,
    receiver_deleted_at timestamptz,
    PRIMARY KEY(message_id)
);


-- ===== VIEWS =====

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

