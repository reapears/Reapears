-- Add down migration script here
-- Add down migration script here

-- Setup Down Migrations

DROP VIEW IF EXISTS services.active_locations;
DROP VIEW IF EXISTS services.active_farms;
DROP VIEW IF EXISTS services.active_harvests;

DROP TABLE IF EXISTS features.message_status;
DROP TABLE IF EXISTS features.direct_messages;
DROP TABLE IF EXISTS features.harvest_subscriptions;
DROP TABLE IF EXISTS services.harvests_wishlist;
DROP TABLE IF EXISTS services.harvests;
-- DROP TABLE IF EXISTS services.location_tags;
DROP TABLE IF EXISTS services.locations;
DROP TABLE IF EXISTS services.regions;
DROP TABLE IF EXISTS services.countries;
DROP TABLE IF EXISTS services.farm_ratings;
-- DROP TABLE IF EXISTS services.farm_tags;
DROP TABLE IF EXISTS services.farms;
-- DROP TABLE IF EXISTS services.cultivar_tags;
DROP TABLE IF EXISTS services.cultivars;
DROP TABLE IF EXISTS services.cultivar_categories;
-- DROP TABLE IF EXISTS services.tags;

DROP TABLE IF EXISTS auth.api_tokens;
DROP TABLE IF EXISTS auth.password_reset_tokens;
DROP TABLE IF EXISTS auth.sessions;
DROP TABLE IF EXISTS accounts.email_pending_updates;
DROP TABLE IF EXISTS accounts.account_delete_requests;
DROP TABLE IF EXISTS accounts.follows;
DROP TABLE IF EXISTS accounts.phones;
DROP TABLE IF EXISTS accounts.emails;
DROP TABLE IF EXISTS accounts.government_ids;
DROP TABLE IF EXISTS accounts.user_profiles;
DROP TABLE IF EXISTS accounts.users;


DROP SCHEMA IF EXISTS archives;
DROP SCHEMA IF EXISTS services;
DROP SCHEMA IF EXISTS auth;
DROP SCHEMA IF EXISTS accounts;
DROP SCHEMA IF EXISTS models;