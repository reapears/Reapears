//! Server settings definitions

//! TODO: Clean up these function once lazy
//! initialization is stabilized in stable rust

/// Server domain name
pub const SERVER_DOMAIN: &str = "http://localhost:3000";

// ---Paths---

/// Server home directory
pub const HOME_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// Server media files root directory
#[must_use]
pub fn media_root() -> String {
    format!("{HOME_DIR}/media")
}

/// Server static files root directory
#[must_use]
pub fn static_root() -> String {
    format!("{HOME_DIR}/static")
}

/// User profile photo directory
#[must_use]
pub fn user_uploads_dir() -> String {
    format!("{}/uploads/user", media_root())
}

/// Cultivar image files directory
#[must_use]
pub fn cultivar_uploads_dir() -> String {
    format!("{}/uploads/cultivar", media_root())
}

/// Harvest image files directory
#[must_use]
pub fn harvest_uploads_dir() -> String {
    format!("{}/uploads/harvest", media_root())
}

/// File not found html response
#[must_use]
pub fn upload_not_found_file() -> String {
    format!("{}/uploads/upload_not_found.html", media_root())
}
