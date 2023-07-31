//! Server configuration impls

use std::{env, fmt, net::SocketAddr};

use axum_extra::extract::cookie::Key;

use crate::{APP_DOMAIN_NAME, DEFAULT_SERVER_ADDR, DEFAULT_SERVER_PORT, SERVER_DOMAIN_NAME};

/// Server config values
#[derive(Clone)]
pub struct Config {
    /// Server address is listening on
    pub local_addr: SocketAddr,
    /// Registered server domain name
    pub domain_name: String,

    /// The database connection url
    pub database_url: String,

    /// Outlook smtp email
    pub mail_email: String,
    /// Outlook smtp password
    pub mail_password: String,

    /// Cookie encryption key
    pub cookie_key: Key,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config{..}").finish()
    }
}

impl Config {
    /// Loads server configuration from environmental variables.
    #[must_use]
    pub fn from_env() -> Self {
        let server_address =
            env::var("SERVER_ADDR").unwrap_or_else(|_| DEFAULT_SERVER_ADDR.to_owned());
        let server_port =
            env::var("SERVER_PORT").unwrap_or_else(|_| DEFAULT_SERVER_PORT.to_owned());
        let cookie_key = env::var("COOKIE_KEY").expect("COOKIE_KEY environment variable not set.");
        let domain_name =
            env::var("SERVER_DOMAIN_NAME").unwrap_or_else(|_| APP_DOMAIN_NAME.to_owned());

        // Initialize server domain name
        let _ = SERVER_DOMAIN_NAME.set(domain_name.clone());

        let server_addr = format!("{server_address}:{server_port}");

        Self {
            local_addr: server_addr.parse().unwrap_or_else(|err| {
                panic!("Invalid socket address provided:{server_address}:{server_port}. : {err}",)
            }),

            domain_name,

            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable not set."),

            mail_email: env::var("MAIL_EMAIL").expect("MAIL_EMAIL environment variable not set."),

            mail_password: env::var("MAIL_PASSWORD")
                .expect("MAIL_PASSWORD environment variable not set."),

            cookie_key: Key::try_from(cookie_key.as_bytes())
                .expect("Key too short, cookie key must be at least 64 bytes"),
        }
    }
}
