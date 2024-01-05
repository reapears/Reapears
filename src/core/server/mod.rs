//! Server setup impls

use axum::{
    error_handling::HandleErrorLayer,
    http::{header, Request},
    middleware::from_extractor_with_state,
    response::{IntoResponse, Response},
};
use clap::Parser;
use tokio::{net::TcpListener, signal};
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    catch_panic::CatchPanicLayer, classify::StatusInRangeAsFailures, cors::CorsLayer,
    request_id::RequestId, trace::TraceLayer, ServiceBuilderExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    accounts::user::models::create_unsecure_superuser,
    auth::{api_key::ApiToken, ApiAuthentication},
    endpoint::{EndpointRejection, EndpointResult},
    types::ModelID,
    CONCURRENCY_LIMIT, ONE_SECOND, REQUEST_PER_SEC, SENSITIVE_HEADERS, TIMEOUT_SECS,
};

use cli::{Commands, ConfigCli};
use config::Config;
use maintenance::server_maintenance;
use routers::server_routers;
use state::{DatabaseConnection, ServerState};

mod cli;
mod config;
mod maintenance;
mod routers;
pub mod state;

/// Server listener
///
/// # Panics
///
/// Panics if failed to start a server
pub async fn run() {
    let config = Config::from_env();
    let addr = config.local_addr;
    let state = ServerState::from_config(config).await;

    let app = server_routers()
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .layer(GlobalConcurrencyLimitLayer::new(CONCURRENCY_LIMIT))
                .timeout(TIMEOUT_SECS)
                .buffer(1024)
                .rate_limit(REQUEST_PER_SEC, ONE_SECOND)
                .trim_trailing_slash()
                .sensitive_headers(SENSITIVE_HEADERS)
                .layer(TraceLayer::new(
                    StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
                ))
                // Authenticates api endpoints
                .layer(from_extractor_with_state::<ApiAuthentication, ServerState>(
                    state.clone(),
                ))
                .layer(CorsLayer::permissive()) // Must remove in production ??
                .set_x_request_id(RequestIdGen)
                .propagate_header(header::HeaderName::from_static("x-request-id"))
                .layer(CatchPanicLayer::custom(handle_panic)),
        )
        .with_state(state.clone());

    // RUN MIGRATIONS
    let db = state.database();
    run_migration(db.clone()).await;

    // Create superuser if values given.
    let cli = ConfigCli::parse();
    if let Some(Commands::WithSuperuser { email, password }) = cli.command {
        let id = create_unsecure_superuser(email, password, db.clone()).await;
        let (token, key) = ApiToken::new_for_user(id);
        let _ = token.insert(db.clone()).await.unwrap();
        println!("API_KEY: {key}");
    }

    // RUN MAINTENANCE TASK
    tokio::spawn(server_maintenance(state));

    tracing::debug!("Listening on: {addr}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// =====

// RUN MIGRATIONS
#[cfg(not(feature = "dev"))]
async fn run_migration(db: DatabaseConnection) {
    sqlx::migrate!().run(&db.pool).await.unwrap();
}

// RUN MIGRATIONS
#[cfg(feature = "dev")]
async fn run_migration(db: DatabaseConnection) {
    sqlx::migrate!("tests/migrations")
        .run(&db.pool)
        .await
        .unwrap();
}

// ===== Tracing impls =====

/// Initializes tracing for dev environment
/// that includes console-subscriber
///
/// # Panics
///
/// Panics if failed to install `color_eyre`
#[cfg(feature = "dev")]
pub fn tracing_init() {
    color_eyre::install().unwrap();

    let console_layer = console_subscriber::spawn();

    // EnvFilter
    let default_filters = "reapears=trace,tower_http=trace";
    let filters = EnvFilter::try_from_default_env().unwrap_or_else(|_| default_filters.into());

    // tracing_subscriber::fmt
    let format = tracing_subscriber::fmt::layer()
        .with_file(false)
        .with_target(false)
        .compact();

    tracing_subscriber::registry()
        // add the console layer to the subscriber
        .with(console_layer)
        // add other layers...
        .with(filters)
        .with(format)
        .init();
}

/// Initializes tracing for production environment
#[cfg(not(feature = "dev"))]
pub fn tracing_init() {
    // EnvFilter
    let default_filters = "reapears=trace,tower_http=trace";
    let filters = EnvFilter::try_from_default_env().unwrap_or_else(|_| default_filters.into());

    // tracing_subscriber::fmt
    let format = tracing_subscriber::fmt::layer().json();

    tracing_subscriber::registry()
        .with(filters)
        .with(format)
        .init();
}

// ===== Middleware impls ======

/// Generates request ids
#[derive(Debug, Clone, Copy)]
pub struct RequestIdGen;

impl tower_http::request_id::MakeRequestId for RequestIdGen {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = ModelID::new().to_string().parse().ok()?;
        Some(RequestId::new(request_id))
    }
}

// ===== Errors impls =====

/// Handles errors from middleware
#[allow(clippy::unused_async)]
async fn handle_error(err: tower::BoxError) -> EndpointResult<()> {
    if err.is::<tower::timeout::error::Elapsed>() {
        tracing::error!("Timeout error, request timed out: {}", err);
        return Err(EndpointRejection::RequestTimeout(
            "Request timed out".into(),
        ));
    }

    if err.is::<tower::load_shed::error::Overloaded>() {
        tracing::error!("Load-shed error; service overloaded: {}.", err);
        return Err(EndpointRejection::ServiceUnavailable(
            "Service is overloaded, try again later.".into(),
        ));
    }

    tracing::error!("Internal server error: {}.", err);
    Err(EndpointRejection::internal_server_error())
}

/// Handles panic errors
#[allow(clippy::needless_pass_by_value)]
fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    let err = err
        .downcast_ref::<&str>()
        .map_or_else(|| "Unknown panic error reason", |s| s);
    tracing::error!("Service panicked: {err}");
    EndpointRejection::internal_server_error().into_response()
}

// ===== Graceful Shutdown impls =====

/// Signal handler for initiating graceful shutdown on the server
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::trace!("Signal received, starting graceful shutdown...");
}
