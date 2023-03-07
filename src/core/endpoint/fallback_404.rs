//! Router `NotFound` handler
#![allow(clippy::unused_async)]

use crate::endpoint::EndpointRejection;
use axum::response::IntoResponse;

/// Handlers unknown routers
pub async fn page_not_found() -> impl IntoResponse {
    EndpointRejection::not_found("page")
}

/*
Error-Messages:

    This account does not exist try searching for another

    Sorry, this page isn't available.
    The link you followed may be broken, or the page may have been removed.
    Go back to Instagram.

    Oops!
    We can't seem to find the page you're looking for.

    Hmm...this page does not exist. Try searching for something else.

    Oops! We can't seem to find the page you're looking for. Try searching for something else.
*/
