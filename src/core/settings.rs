//! Server settings definitions

/// Server domain name
pub const SERVER_DOMAIN: &str = "http://localhost:3000";

// ===== Paths ======

/// Server home directory
pub const HOME_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// Server media files root directory
pub const MEDIA_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/media");

/// Server static files root directory
pub const STATIC_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

/// Users profile photo uploads directory
pub const USER_UPLOAD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/media/uploads/user");

/// Cultivars image file uploads directory
pub const CULTIVAR_UPLOAD_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/media/uploads/cultivar");

/// Harvests image file uploads directory
pub const HARVEST_UPLOAD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/media/uploads/harvest");

/// File not found html response
pub const FILE_NOT_FOUND_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/media/uploads/upload_not_found.html"
);
