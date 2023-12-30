//! Email Message impls

use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    Message,
};

use crate::{
    error::{ServerError, ServerResult},
    APP_NAME,
};

/// Email message
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub message: Message,
}

impl EmailMessage {
    /// Create a new email for outlook client
    pub fn from_server(
        from_email: &str,
        email_to: &str,
        subject: &str,
        plain_text: String,
        html: String,
    ) -> ServerResult<Self> {
        Self::write(APP_NAME, from_email, email_to, subject, plain_text, html)
    }

    /// Create new multipart email
    pub fn write(
        display_name: &str,
        from: &str,
        to: &str,
        subject: &str,
        plain_text: String,
        html: String,
    ) -> ServerResult<Self> {
        // Email plain text fallback body
        let plain_text = SinglePart::builder()
            .header(ContentType::TEXT_PLAIN)
            .body(plain_text);

        // Email html body
        let html = SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(html);

        let body = MultiPart::alternative()
            .singlepart(plain_text)
            .singlepart(html);

        let Ok(message) = Message::builder()
            .from(format!("{display_name} <{from}>").parse().unwrap())
            .to(format!("<{to}>").parse().unwrap())
            .subject(subject)
            .multipart(body)
        else {
            return Err(ServerError::new("Failed to build email message"));
        };

        Ok(Self { message })
    }

    /// Create new plain text email message
    pub fn write_plain(from: &str, to: &str, subject: String, body: String) -> ServerResult<Self> {
        let Ok(message) = Message::builder()
            .from(format!("{APP_NAME} <{from}>",).parse().unwrap())
            .to(format!("<{to}>").parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
        else {
            return Err(ServerError::new("Failed to build email message"));
        };
        Ok(Self { message })
    }
}
