//! Email sender

use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor as Tokio,
};

use crate::error::{ServerError, ServerResult};

use super::{EMAIL_DISPLAY_NAME, OUTLOOK_SMTP_SERVER, REAPEARS_EMAIL};

/// SMTP email sender
#[derive(Debug, Clone)]
pub struct Mail {
    mail: AsyncSmtpTransport<Tokio>,
}

impl Mail {
    /// Creates a remote connection to `smtp_server` using STARTTLS
    #[must_use]
    pub fn new(smtp_server: &str, username: String, password: String) -> Self {
        let credentials = Credentials::new(username, password);
        let pool = PoolConfig::default();
        Self {
            mail: AsyncSmtpTransport::<Tokio>::starttls_relay(smtp_server)
                .unwrap()
                .credentials(credentials)
                .pool_config(pool)
                .build(),
        }
    }

    /// Creates a remote connection to outlook smtp server using STARTTLS
    #[must_use]
    pub fn outlook(password: String) -> Self {
        Self::new(OUTLOOK_SMTP_SERVER, REAPEARS_EMAIL.to_owned(), password)
    }

    /// Sends an email
    pub async fn send(&self, email: EmailMessage) -> ServerResult<()> {
        match self.mail.send(email.message).await {
            Ok(_response) => Ok(()),
            Err(err) => {
                tracing::error!("Sending email error: {}", err);
                Err(ServerError::internal(Box::new(err)))
            }
        }
    }
}

/// Email message
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub message: Message,
}

impl EmailMessage {
    /// Create a new email for outlook client
    pub fn from_outlook(
        to: &str,
        subject: &str,
        plain_text: String,
        html: String,
    ) -> ServerResult<Self> {
        Self::write(
            EMAIL_DISPLAY_NAME,
            REAPEARS_EMAIL,
            to,
            subject,
            plain_text,
            html,
        )
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
            else{
                return  Err(ServerError::new("Failed to build email message"));
            };

        Ok(Self { message })
    }

    /// Create new plain text email message
    pub fn write_plain(from: &str, to: &str, subject: String, body: String) -> ServerResult<Self> {
        let Ok(message) = Message::builder()
            .from(format!("{EMAIL_DISPLAY_NAME} <{from}>").parse().unwrap())
            .to(format!("<{to}>").parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)else{
            return  Err(ServerError::new("Failed to build email message"));
        };
        Ok(Self { message })
    }
}
