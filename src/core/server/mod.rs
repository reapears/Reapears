//! Server setup impls

use axum::http::header;

mod config;
mod maintenance;
mod routers;
mod serve;
pub mod state;

use routers::server_routers;

pub use serve::{serve, tracing_init};

/// Number of inflight request allowed on the server
const CONCURRENCY_LIMIT: usize = 2048;

/// Request timeout seconds
const TIMEOUT_SECS: u64 = 60 * 3; // three minute

/// Headers that should not appear in longs
const SENSITIVE_HEADERS: [header::HeaderName; 4] = [
    header::AUTHORIZATION,
    header::PROXY_AUTHORIZATION,
    header::COOKIE,
    header::SET_COOKIE,
];

/// Database max connection number
pub const DATABASE_MAX_CONNECTIONS: u32 = 20;
