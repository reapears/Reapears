use reapears::server::tracing_init;

#[tokio::main]
async fn main() {
    /*
    How to run
    ON Unix:
    -$ DATABASE_URL="..." cargo run --release --server_addr
    ON Windows:
    $Env:DATABASE_URL="..."; cargo run  --release -- --cookie-key .. --
    */

    dotenvy::dotenv().unwrap();

    tracing_init();
    reapears::serve().await;
}
