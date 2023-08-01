//! Reapears entry point.

use reapears::server::{self, tracing_init};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_init();

    server::run().await;
}
