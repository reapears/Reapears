#![allow(clippy::unused_async)]

use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::{header, Request},
};
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    cors::CorsLayer,
    request_id::{PropagateRequestIdLayer, RequestId},
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::TraceLayer,
    BoxError, ServiceBuilderExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

use crate::endpoint::{EndpointRejection, EndpointResult};

use super::{
    config::Config, server_routers, state::ServerState, CONCURRENCY_LIMIT, SENSITIVE_HEADERS,
    TIMEOUT_SECS,
};

/// Starts and serve an application
///
/// # Panics
///
/// Panics if failed to start a server
pub async fn serve() {
    // cors, compression

    let config = Config::load();
    let addr = config.server_addr;
    let state = ServerState::from_config(config).await;

    let app_server = server_routers(state)
        .layer(
            ServiceBuilder::new()
                .layer(GlobalConcurrencyLimitLayer::new(CONCURRENCY_LIMIT))
                .layer(TraceLayer::new_for_http())
                .set_x_request_id(RequestIdGen)
                .layer(HandleErrorLayer::new(loadshed_error_handler))
                .load_shed()
                .layer(HandleErrorLayer::new(timeout_error_handler))
                .timeout(Duration::from_secs(TIMEOUT_SECS)),
        )
        .layer(CorsLayer::permissive())
        .layer(PropagateRequestIdLayer::new(
            header::HeaderName::from_static("x-request-id"),
        ))
        .layer(SetSensitiveHeadersLayer::from_shared(Arc::new(
            SENSITIVE_HEADERS,
        )));

    tracing::debug!("Listening on: {addr}");
    axum::Server::bind(&addr)
        .serve(app_server.into_make_service())
        .await
        .unwrap();
}

// Tracing initialization impls

/// Initializes server tracing subscriber
///
/// # Panics
///
/// Panics if failed to install `color_eyre`
pub fn tracing_init() {
    color_eyre::install().unwrap();

    let console_layer = console_subscriber::spawn();

    // EnvFilter
    let default_filters = "reapears=trace,tower_http=trace";
    let filters = EnvFilter::try_from_default_env().unwrap_or_else(|_| default_filters.into());

    // tracing_subscriber::fmt
    let format = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        // add the console layer to the subscriber
        .with(console_layer)
        // add other layers...
        .with(filters)
        .with(format)
        .init();
}

// ---RequestId impls---

#[derive(Clone, Copy)]
pub struct RequestIdGen;

impl tower_http::request_id::MakeRequestId for RequestIdGen {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().ok()?;
        Some(RequestId::new(request_id))
    }
}

// ---Middleware error handlers impls---

async fn timeout_error_handler(err: BoxError) -> EndpointResult<()> {
    if err.is::<tower::timeout::error::Elapsed>() {
        tracing::error!("Timeout error, request could be processed: {}", err);
        Err(EndpointRejection::RequestTimeout(
            "Request took too long".into(),
        ))
    } else {
        Err(EndpointRejection::internal_server_error())
    }
}

async fn loadshed_error_handler(err: BoxError) -> EndpointResult<()> {
    tracing::error!("Load-shed error, request could be processed: {}", err);
    Err(EndpointRejection::internal_server_error())
}
