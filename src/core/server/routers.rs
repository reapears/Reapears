#![allow(clippy::doc_markdown, rustdoc::broken_intra_doc_links)]

//! Server Api Routers
//!
//! Endpoints                                                                          Method(s) Allowed                Login Required            Admin
//!
//! [::]/api/v1/account/signup                                                         POST
//! [::]/api/v1/account/deactivate                                                     POST
//! [::]/api/v1/account/login                                                          POST
//! [::]/api/v1/account/logout                                                         DELETE
//! [::]/api/v1/account/lock                                                           POST
//! [::]/api/v1/account/unlock                                                         POST
//! [::]/api/v1/account/confirm?token=...                                              GET
//! [::]/api/v1/account/email-exists                                                   POST
//! [::]/api/v1/account/forgot-password                                                POST
//! [::]/api/v1/account/reset-password?token=...                                       POST
//!
//! [::]/api/v1/account/users                                                          GET
//! [::]/api/v1/account/users/:user_id/profile                                         GET
//! [::]/api/v1/account/users/profile                                                  GET, PUT
//! [::]/api/v1/account/users/profile/photo                                            POST, DELETE
//!
//! [::]/api/v1/account/settings/personal-info                                         GET, PUT,
//! [::]/api/v1/account/settings/change-email                                          POST
//! [::]/api/v1/account/settings/verify-email                                          POST
//! [::]/api/v1/account/settings/change-password                                       POST
//! [::]/api/v1/account/settings/verify-password                                       POST
//!
//! [::]/api/v1/cultivars                                                               GET, POST
//! [::]/api/v1/cultivars/:cultivar_id                                                  GET, PUT, DELETE
//! [::]/api/v1/cultivars/index                                                         GET
//! [::]/api/v1/cultivars/categories                                                    GET, POST
//! [::]/api/v1/cultivars/categories/:category_id                                       PUT, DELETE
//! [::]/api/v1/cultivars/:cultivar_id/photo                                            POST, DELETE
//!
//! [::]/api/v1/harvests                                                                GET POST
//! [::]/api/v1/harvests/:harvest_id                                                    GET, PUT, DELETE
//! [::]/api/v1/harvests/:harvest_id/photos                                             POST, DELETE
//!
//! [::]/api/v1/farms                                                                   GET POST
//! [::]/api/v1/farms/:farm_id                                                          GET, PUT, DELETE
//! [::]/api/v1/farms/:farm_id/locations                                                GET, POST
//! [::]/api/v1/farms/:farm_id/ratings                                                  GET, POST
//! [::]/api/v1/farms/ratings/:rating_id                                                GET, PUT, DELETE
//!
//! [::]/api/v1/locations                                                               GET
//! [::]/api/v1/locations/:location_id                                                  GET, PUT, DELETE
//! [::]/api/v1/locations/countries                                                     GET, POST
//! [::]/api/v1/locations/countries/country_id                                          PUT, DELETE
//! [::]/api/v1/locations/countries/:country_id/regions                                 GET, POST
//! [::]/api/v1/locations/countries/regions/region_id                                   PUT DELETE
//!
//!
//! --------------------------------------------------------------
//!
//!
//                                           POST, DELETE
//!
//! [::]/api/v1/messages                                                                POST, DELETE
//! [::]/api/v1/messages/chat                                                           POST, DELETE
//! [::]/api/v1/messages/conversation                                                   POST, DELETE

/*


 [::]/api/v1/direct-message/                                           GET, POST



 [::]/farmer/                                                           GET
 [::]/farmer/:farm_id/add-product/                                     POST
 produce
 [::]/became-a-farmer                                                  POST

 https://www.airbnb.com/become-a-host
 users/show/:id ??

*/

use std::path::Path;

use axum::{
    http::{
        header::{self, HeaderValue},
        StatusCode,
    },
    routing::MethodRouter,
    routing::{get, get_service},
    Router,
};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeader,
};

use crate::{
    endpoint::EndpointResult,
    settings::{
        CULTIVAR_UPLOAD_DIR, HARVEST_UPLOAD_DIR, USER_UPLOAD_DIR, WEB_APP_BUILD_DIR, WEB_APP_ROOT,
    },
};

use super::state::ServerState;

mod accounts;
mod services;

#[allow(clippy::declare_interior_mutable_const)]
const MAX_AGE_ONE_DAY: HeaderValue = HeaderValue::from_static("public, max-age=86400");
#[allow(clippy::declare_interior_mutable_const)]
const MAX_AGE_ONE_YEAR: HeaderValue = HeaderValue::from_static("public, max-age=31536000");

pub fn server_routers() -> Router<ServerState> {
    assert!(
        Path::new(WEB_APP_BUILD_DIR).exists(),
        "Web app not build. expected at {:?}",
        Path::new(WEB_APP_BUILD_DIR)
    );

    Router::new()
        .fallback_service(web_app())
        .nest("/api/v1", api_v1_router())
        .route("/health-check", get(health_check))
}

/// Api version one routers
fn api_v1_router() -> Router<ServerState> {
    Router::new()
        .merge(services::routers())
        .merge(accounts::routers())
        .merge(pictures_router())
}

/// Verifies the server is up and ready to receive incoming requests.
#[allow(clippy::unused_async)]
async fn health_check() -> EndpointResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Sets up `Webapp` frontend service
fn web_app() -> MethodRouter {
    let frontend = ServeDir::new(WEB_APP_BUILD_DIR)
        .fallback(ServeFile::new(WEB_APP_ROOT))
        .precompressed_gzip();
    let with_caching =
        SetResponseHeader::if_not_present(frontend, header::CACHE_CONTROL, MAX_AGE_ONE_YEAR);
    get_service(with_caching)
}

/// Sets up `ServeDir` service
fn serve_dir(path: impl AsRef<Path>, max_age: HeaderValue) -> MethodRouter {
    let files = ServeDir::new(path).precompressed_gzip();
    let with_caching = SetResponseHeader::if_not_present(files, header::CACHE_CONTROL, max_age);
    get_service(with_caching)
}

/// Picture routes
fn pictures_router() -> Router<ServerState> {
    Router::new()
        .nest_service(
            "/cultivars/p",
            get_service(serve_dir(CULTIVAR_UPLOAD_DIR, MAX_AGE_ONE_DAY)),
        )
        .nest_service(
            "/harvests/p",
            get_service(serve_dir(HARVEST_UPLOAD_DIR, MAX_AGE_ONE_DAY)),
        )
        .nest_service(
            "/account/users/photo",
            get_service(serve_dir(USER_UPLOAD_DIR, MAX_AGE_ONE_DAY)),
        )
}
