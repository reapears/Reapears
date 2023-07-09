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

use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

use super::state::ServerState;
use crate::{
    endpoint::EndpointResult,
    settings::{CULTIVAR_UPLOAD_DIR, FILE_NOT_FOUND_PATH, HARVEST_UPLOAD_DIR, USER_UPLOAD_DIR},
};

mod accounts;
mod services;

pub fn server_routers() -> Router<ServerState> {
    Router::new()
        .route("/health-check", get(health_check))
        .merge(services::routers())
        .merge(accounts::routers())
        .merge(pictures_router())
}

/// Verifies the server is up and ready to receive incoming requests.
#[allow(clippy::unused_async)]
async fn health_check() -> EndpointResult<StatusCode> {
    Ok(StatusCode::OK)
}

fn pictures_router() -> Router<ServerState> {
    let cultivar_dir =
        get_service(ServeDir::new(CULTIVAR_UPLOAD_DIR).not_found_service(file_not_found()));

    let harvest_dir =
        get_service(ServeDir::new(HARVEST_UPLOAD_DIR).not_found_service(file_not_found()));

    let user_dir = get_service(ServeDir::new(USER_UPLOAD_DIR).not_found_service(file_not_found()));

    Router::new()
        .nest_service("/cultivars/p", cultivar_dir)
        .nest_service("/harvests/p", harvest_dir)
        .nest_service("/account/users/photo", user_dir)
}

/// File not found error response
fn file_not_found() -> ServeFile {
    ServeFile::new(FILE_NOT_FOUND_PATH)
}
