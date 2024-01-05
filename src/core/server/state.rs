//! Server State impls

use std::{fmt, sync::Arc};

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{features::direct_message::ChatFeed, mail::Mail};

use super::config::Config;

/// Server's state
#[derive(Clone)]
pub struct ServerState(Arc<StateInner>);

#[derive(Clone)]
struct StateInner {
    database: DatabaseConnection,
    outlook_client: Mail,
    chat: ChatFeed,
    cookie_key: Key,
}

impl ServerState {
    /// Creates new `ServerState`.
    pub async fn from_config(config: Config) -> Self {
        Self(Arc::new(StateInner {
            database: DatabaseConnection::new(&config.database_url).await,
            outlook_client: Mail::outlook(&config.mail_email, config.mail_password),
            chat: ChatFeed::new(),
            cookie_key: config.cookie_key,
        }))
    }

    /// Clone and returns database connection
    #[must_use]
    #[inline]
    pub fn database(&self) -> DatabaseConnection {
        self.0.database.clone()
    }

    /// Clone and returns mail client
    #[must_use]
    #[inline]
    pub fn outlook_client(&self) -> Mail {
        self.0.outlook_client.clone()
    }

    /// Clone and returns chat feed instance
    #[must_use]
    #[inline]
    pub fn chat_feed(&self) -> ChatFeed {
        self.0.chat.clone()
    }

    /// Clone and returns chat feed instance
    #[must_use]
    #[inline]
    pub fn chat_feed_as_ref(&self) -> &ChatFeed {
        &self.0.chat
    }

    /// Clone and returns cookie key
    #[must_use]
    #[inline]
    pub fn cookie_key(&self) -> Key {
        self.0.cookie_key.clone()
    }
}

impl fmt::Debug for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerState{..}").finish()
    }
}

impl FromRef<ServerState> for DatabaseConnection {
    fn from_ref(state: &ServerState) -> Self {
        state.database()
    }
}

impl FromRef<ServerState> for ChatFeed {
    fn from_ref(state: &ServerState) -> Self {
        state.chat_feed()
    }
}

impl FromRef<ServerState> for Mail {
    fn from_ref(state: &ServerState) -> Self {
        state.outlook_client()
    }
}

impl FromRef<ServerState> for Key {
    fn from_ref(state: &ServerState) -> Self {
        state.cookie_key()
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
