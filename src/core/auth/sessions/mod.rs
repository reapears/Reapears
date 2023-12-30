//! User sessions impls

use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};

use crate::auth::{cookies::SESSION_TOKEN, hash_token, TokenHash};

pub mod db;
pub mod forms;
pub mod handlers;
pub mod models;
mod utils;

/// Gets session token hash from the cookie jar
#[must_use]
pub fn get_session_token_hash(jar: &PrivateCookieJar) -> Option<TokenHash> {
    let token: Option<TokenHash> = jar
        .get(SESSION_TOKEN)
        .map(|cookie| cookie.value().to_owned())
        .map(|token| hash_token(token.as_bytes()));
    token
}

/// Adds session cookies into cookie jar
#[must_use]
fn add_session_cookie(jar: PrivateCookieJar, token: String) -> PrivateCookieJar {
    let mut token_cookie = Cookie::new(SESSION_TOKEN, token);
    token_cookie.set_path("/");

    jar.add(token_cookie)
}

/// Removes session cookie from cookie jar
#[must_use]
fn remove_session_cookie(jar: PrivateCookieJar) -> PrivateCookieJar {
    let jar = jar.remove(Cookie::build(SESSION_TOKEN));
    jar
}
