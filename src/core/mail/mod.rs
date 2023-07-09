//! SMTP email sender impls

mod emails;
mod mailer;
mod message;

pub use mailer::Mail;

/// Microsoft outlook mail smtp STARTTLS server
const OUTLOOK_SMTP_SERVER: &str = "smtp.office365.com";

/// Microsoft outlook email
const REAPEARS_EMAIL: &str = "reapears@outlook.com";

/// Email display name
const EMAIL_DISPLAY_NAME: &str = "Reapears";
