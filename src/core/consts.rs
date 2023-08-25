//! Constantans variable impls

use std::{sync::OnceLock, time::Duration};

use axum::http::header;
use image::ImageFormat;

/// ===== APP =====

/// App name.
pub const APP_NAME: &str = "Reapears";
/// Hard coded registered server domain name.
/// For production use. `https://reapears.com` .
/// Use `SERVER_DOMAIN_NAME` instead works for both.
pub const APP_DOMAIN_NAME: &str = "https://reapears.com";
/// Server domain name, In production points to `https://reapears.com` .
pub static SERVER_DOMAIN_NAME: OnceLock<String> = OnceLock::new();
/// The default socket address the server is listening on.
pub const DEFAULT_SERVER_ADDR: &str = "127.0.0.1";
/// The default socket post the server is listening on.
pub const DEFAULT_SERVER_PORT: &str = "4000";

// ===== ACCOUNT =====

/// Password token expiry time
pub const PASSWORD_TOKEN_EXPIRY_TIME: i64 = 30; // 30 minutes
/// Account confirmation token expiry time
pub const ACCOUNT_CONFIRM_TOKEN_EXPIRY: i64 = 30; // minutes
/// Number of profile photos allowed per user
pub const USER_MAX_PROFILE_PHOTO: u8 = 1;
/// Max numbers of days a user has before their account deleted permanently.
pub const MAX_DAYS_TO_DELETE_ACCOUNT: u8 = 90;

// ===== AUTH =====

/// Endpoints that are not protected with an API key;
pub const UNAUTHENTICATED_ENDPOINTS: [&str; 6] = [
    // "/account/signup" ??
    "/account/confirm",
    "/health-check",
    "/account/reset-password",
    // Media endpoints
    "/harvests/p",
    "/cultivars/p",
    "/account/users/photo",
];
/// An error message for when a user entered a wrong password of username
pub const INVALID_CREDENTIALS_ERR_MSG: &str = "The username or password you provided is incorrect.";

// ===== FILES =====

/// Image formats that can be uploaded on the server.
pub const SUPPORTED_UPLOAD_IMAGE_FORMATS: [&str; 4] = ["jpeg", "jpg", "png", "webp"];
/// Image format saved on the server.
pub const IMAGE_OUTPUT_FORMATS: [ImageFormat; 2] = [ImageFormat::Jpeg, ImageFormat::WebP];
/// Image maximum size allowed on the server
pub const IMAGE_MAX_SIZE: usize = 20 * 1024 * 1024; // 20 * 1024 * 1024 = 20mb

// ===== MAIL =====

/// Microsoft outlook mail smtp STARTTLS server
pub const OUTLOOK_SMTP_SERVER: &str = "smtp.office365.com";

// ===== SERVER =====

/// Number of inflight request allowed on the server
pub const CONCURRENCY_LIMIT: usize = 2048;
/// Number of request per seconds allowed.
pub const REQUEST_PER_SEC: u64 = 500;
/// Database max connection number.
pub const DATABASE_MAX_CONNECTIONS: u32 = 20;
/// Request timeout seconds. 2(two) minutes.
pub const TIMEOUT_SECS: Duration = Duration::from_secs(60 * 2);
/// Duration of one second
pub const ONE_SECOND: Duration = Duration::from_secs(1);
/// Headers that should not appear in longs
pub const SENSITIVE_HEADERS: [header::HeaderName; 4] = [
    header::AUTHORIZATION,
    header::PROXY_AUTHORIZATION,
    header::COOKIE,
    header::SET_COOKIE,
];

// ===== SERVICES =====

/// Number of images allowed to be uploaded per cultivar
pub const CULTIVAR_MAX_IMAGE: u8 = 1;
/// Determine for how long the harvest should be on the
/// platform before it's archived.
/// If the harvest has been on the platform for
/// less-than these days it will be deleted.
pub const HARVEST_MAX_AGE_TO_ARCHIVE: i64 = 4; // days
/// Number of images allowed to be uploaded per harvest
pub const HARVEST_MAX_IMAGE: u8 = 5;
