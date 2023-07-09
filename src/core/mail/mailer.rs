//! Email sender

use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor as Tokio,
};

use crate::error::{ServerError, ServerResult};

use super::{emails::EmailTemplates, message::EmailMessage, OUTLOOK_SMTP_SERVER, REAPEARS_EMAIL};

/// SMTP email sender
#[derive(Debug, Clone)]
pub struct Mail {
    mail: AsyncSmtpTransport<Tokio>,
    emails: EmailTemplates,
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
            emails: EmailTemplates::new(),
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

    /// Return account confirm email
    pub fn account_confirm(
        &self,
        first_name: &str,
        user_email: &str,
        link: &str,
    ) -> ServerResult<EmailMessage> {
        self.emails.account_confirm(first_name, user_email, link)
    }

    /// Return approve email change email
    pub fn approve_email_change(
        &self,
        first_name: &str,
        user_email: &str,
        new_email: &str,
        code: &str,
    ) -> ServerResult<EmailMessage> {
        self.emails
            .approve_email_change(first_name, user_email, new_email, code)
    }

    /// Return password reset email
    pub fn password_reset(
        &self,
        first_name: &str,
        user_email: &str,
        link: &str,
    ) -> ServerResult<EmailMessage> {
        self.emails.password_reset(first_name, user_email, link)
    }

    /// Return verify new-email email
    pub fn verify_new_email(
        &self,
        first_name: &str,
        new_email: &str,
        code: &str,
    ) -> ServerResult<EmailMessage> {
        self.emails.verify_new_email(first_name, new_email, code)
    }
}
