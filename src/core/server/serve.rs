//! Sever setup impls

use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    http::{header, Request},
    middleware::from_extractor_with_state,
};
use tokio::signal;
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    classify::StatusInRangeAsFailures, cors::CorsLayer, request_id::RequestId, trace::TraceLayer,
    ServiceBuilderExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    auth::ApiAuthentication,
    endpoint::{EndpointRejection, EndpointResult},
    types::ModelID,
};

use super::{
    config::Config, maintenance::server_maintenance, server_routers, state::ServerState,
    CONCURRENCY_LIMIT, SENSITIVE_HEADERS, TIMEOUT_SECS,
};

/// Server listener
///
/// # Panics
///
/// Panics if failed to start a server
pub async fn serve() {
    let config = Config::load();
    let addr = config.server_addr;
    let state = ServerState::from_config(config).await;

    let app_server = server_routers()
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .layer(GlobalConcurrencyLimitLayer::new(CONCURRENCY_LIMIT))
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .buffer(1024)
                .rate_limit(500, Duration::from_secs(1))
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
                .catch_panic(),
        )
        .with_state(state.clone());

    // * RUN MAINTENANCE TASK *
    tokio::spawn(server_maintenance(state));

    tracing::debug!("Listening on: {addr}");
    axum::Server::bind(&addr)
        .serve(app_server.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// ==== Tracing impls =====

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
        .pretty();

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
    let format = tracing_subscriber::fmt::layer()
        .with_file(false)
        .with_target(false)
        .pretty();

    tracing_subscriber::registry()
        .with(filters)
        .with(format)
        .init();
}

// ===== Middleware impls ======

/// Generates request ids
#[derive(Clone, Copy)]
pub struct RequestIdGen;

impl tower_http::request_id::MakeRequestId for RequestIdGen {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = ModelID::new().to_string().parse().ok()?;
        Some(RequestId::new(request_id))
    }
}

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

// ===== Graceful Shutdown impl =====

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
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::trace!("Signal received, starting graceful shutdown...");
}
