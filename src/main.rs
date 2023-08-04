//! Reapears entry point.

use reapears::server::{self, tracing_init};

fn main() {
    let _ = dotenvy::dotenv();
    tracing_init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            server::run().await;
        });
}
