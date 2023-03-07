//! SMTP email sender impls

pub mod emails;
mod mailer;

pub use mailer::{EmailMessage, Mail};

/// Microsoft outlook mail smtp STARTTLS server
const OUTLOOK_SMTP_SERVER: &str = "smtp.office365.com";

/// Microsoft outlook email
const REAPEARS_EMAIL: &str = "reapears@outlook.com";

/// Email display name
const EMAIL_DISPLAY_NAME: &str = "Reapears";
