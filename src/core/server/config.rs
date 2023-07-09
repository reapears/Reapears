//! Server configuration
#![allow(missing_debug_implementations)]

use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use axum_extra::extract::cookie::Key;
use clap::Parser;

/// Server config values
#[derive(Clone)]
pub struct Config {
    /// Server address in listening on
    pub server_addr: SocketAddr,

    /// The database connection url
    pub database_url: String,

    /// Outlook smtp password
    pub outlook_password: String,

    /// Cookie encryption key
    pub cookie_key: Key,
}

impl Config {
    /// Loads configuration values
    #[must_use]
    pub fn load() -> Self {
        let config = ConfigCli::parse();
        let database_url = config
            .database_url
            .unwrap_or_else(|| env::var("DATABASE_URL").expect("DATABASE_URL is not set."));
        Self {
            server_addr: config
                .server_addr
                .parse()
                .expect("failed to parse socket address"),
            database_url,
            cookie_key: Key::try_from(config.cookie_key.as_bytes())
                .expect("Key too short, cookie key must be at least 64 bytes"),
            outlook_password: config.outlook_password,
        }
    }

    /// Gets the ip address the server is running on
    pub const fn host(&self) -> IpAddr {
        self.server_addr.ip()
    }

    /// Gets the port of the server is running on
    pub const fn port(&self) -> u16 {
        self.server_addr.port()
    }
}

// ---cli---

/// Collect config values from cli
#[derive(Clone, Parser)]
pub struct ConfigCli {
    #[arg(default_value = "127.0.0.1:3000")]
    pub server_addr: String,
    #[arg(long)]
    pub database_url: Option<String>,
    #[arg(long)]
    pub outlook_password: String,
    #[arg(long)]
    pub cookie_key: String,
}
