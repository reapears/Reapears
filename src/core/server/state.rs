//! Server State impls

use std::fmt;

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{features::direct_message::ChatFeed, mail::Mail};

use super::config::Config;

/// Server's state
#[derive(Clone)]
pub struct ServerState {
    pub database: DatabaseConnection,
    pub outlook_client: Mail,
    pub chat: ChatFeed,
    pub cookie_key: Key,
}

impl ServerState {
    /// Creates new `ServerState`.
    pub async fn from_config(config: Config) -> Self {
        Self {
            database: DatabaseConnection::new(&config.database_url).await,
            outlook_client: Mail::outlook(&config.mail_email, config.mail_password),
            chat: ChatFeed::new(),
            cookie_key: config.cookie_key,
        }
    }
}

impl fmt::Debug for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerState{..}").finish()
    }
}

impl FromRef<ServerState> for DatabaseConnection {
    fn from_ref(state: &ServerState) -> Self {
        state.database.clone()
    }
}

impl FromRef<ServerState> for ChatFeed {
    fn from_ref(state: &ServerState) -> Self {
        state.chat.clone()
    }
}

impl FromRef<ServerState> for Mail {
    fn from_ref(state: &ServerState) -> Self {
        state.outlook_client.clone()
    }
}

impl FromRef<ServerState> for Key {
    fn from_ref(state: &ServerState) -> Self {
        state.cookie_key.clone()
    }
}

// ===== Database impls ======

/// Postgres database connection
#[derive(Clone, Debug)]
pub struct DatabaseConnection {
    pub pool: PgPool,
}

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(crate::DATABASE_MAX_CONNECTIONS)
            .connect(database_url)
            .await
            .expect("Failed to connect to the database.");
        Self { pool }
    }
}
