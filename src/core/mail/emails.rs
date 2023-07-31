//! Emails impls
use minijinja::context;

use super::message::EmailMessage;
use crate::{error::ServerResult, APP_NAME};

// ====== Email Templates ======

/// An email to user to confirm their email on signup.
const ACCOUNT_CONFIRMATION_EMAIL_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/confirm_account.html"
));
/// An email to user to confirm their email on signup.
const ACCOUNT_CONFIRMATION_EMAIL_TEXT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/confirm_account.txt"
));

/// An email to user to approve email change.
const APPROVE_EMAIL_CHANGE_EMAIL_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/approve_email_change.html"
));
/// An email to user to approve email change.
const APPROVE_EMAIL_CHANGE_EMAIL_TEXT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/approve_email_change.txt"
));

/// An email to user to reset their password on forgot password.
const PASSWORD_RESET_EMAIL_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/password_reset.html"
));
/// An email to user to reset their password on forgot password.
const PASSWORD_RESET_EMAIL_TEXT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/password_reset.txt"
));

/// An email to user to verify the new email they want to change to.
const VERIFY_NEW_EMAIL_CHANGE_EMAIL_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/verify_new_email.html"
));
/// An email to user to verify the new email they want to change to.
const VERIFY_NEW_EMAIL_CHANGE_EMAIL_TEXT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/templates/emails/verify_new_email.txt"
));

// ===== Email Template Names =====
// Names used to identify templates in the email template container

const NAME_ACCOUNT_CONFIRMATION_EMAIL_HTML: &str = "confirm_account_html";
const NAME_ACCOUNT_CONFIRMATION_EMAIL_TEXT: &str = "confirm_account_txt";

const NAME_APPROVE_EMAIL_CHANGE_EMAIL_HTML: &str = "approve_email_change_html";
const NAME_APPROVE_EMAIL_CHANGE_EMAIL_TEXT: &str = "approve_email_change_txt";

const NAME_PASSWORD_RESET_EMAIL_HTML: &str = "password_reset_html";
const NAME_PASSWORD_RESET_EMAIL_TEXT: &str = "password_reset_txt";

const NAME_VERIFY_NEW_EMAIL_CHANGE_EMAIL_HTML: &str = "verify_new_email_html";
const NAME_VERIFY_NEW_EMAIL_CHANGE_EMAIL_TEXT: &str = "verify_new_email_txt";

/// A container for email templates
#[derive(Debug, Clone)]
pub struct EmailTemplates(minijinja::Environment<'static>);

impl Default for EmailTemplates {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailTemplates {
    /// Creates a new email templates container
    #[must_use]
    pub fn new() -> Self {
        let mut env = minijinja::Environment::new();

        env.add_template(
            NAME_ACCOUNT_CONFIRMATION_EMAIL_HTML,
            ACCOUNT_CONFIRMATION_EMAIL_HTML,
        )
        .unwrap();
        env.add_template(
            NAME_ACCOUNT_CONFIRMATION_EMAIL_TEXT,
            ACCOUNT_CONFIRMATION_EMAIL_TEXT,
        )
        .unwrap();

        env.add_template(
            NAME_APPROVE_EMAIL_CHANGE_EMAIL_HTML,
            APPROVE_EMAIL_CHANGE_EMAIL_HTML,
        )
        .unwrap();
        env.add_template(
            NAME_APPROVE_EMAIL_CHANGE_EMAIL_TEXT,
            APPROVE_EMAIL_CHANGE_EMAIL_TEXT,
        )
        .unwrap();

        env.add_template(NAME_PASSWORD_RESET_EMAIL_HTML, PASSWORD_RESET_EMAIL_HTML)
            .unwrap();
        env.add_template(NAME_PASSWORD_RESET_EMAIL_TEXT, PASSWORD_RESET_EMAIL_TEXT)
            .unwrap();

        env.add_template(
            NAME_VERIFY_NEW_EMAIL_CHANGE_EMAIL_HTML,
            VERIFY_NEW_EMAIL_CHANGE_EMAIL_HTML,
        )
        .unwrap();
        env.add_template(
            NAME_VERIFY_NEW_EMAIL_CHANGE_EMAIL_TEXT,
            VERIFY_NEW_EMAIL_CHANGE_EMAIL_TEXT,
        )
        .unwrap();

        Self(env)
    }

    /// Return account confirm email
    pub fn account_confirm(
        &self,
        server_email: &str,
        first_name: &str,
        user_email: &str,
        link: &str,
    ) -> ServerResult<EmailMessage> {
        let text = self
            .0
            .get_template(NAME_ACCOUNT_CONFIRMATION_EMAIL_TEXT)
            .unwrap()
            .render(context! { first_name => first_name, email => user_email, link => link })
            .unwrap();
        let html = self
            .0
            .get_template(NAME_ACCOUNT_CONFIRMATION_EMAIL_HTML)
            .unwrap()
            .render(context! { first_name => first_name, email => user_email, link => link })
            .unwrap();

        let subject = format!("[{APP_NAME}] Please verify your email address.");

        EmailMessage::from_server(server_email, user_email, &subject, text, html)
    }

    /// Return approve email change email
    pub fn approve_email_change(
        &self,
        server_email: &str,
        first_name: &str,
        user_email: &str,
        new_email: &str,
        code: &str,
    ) -> ServerResult<EmailMessage> {
        let text = self
            .0
            .get_template(NAME_APPROVE_EMAIL_CHANGE_EMAIL_TEXT)
            .unwrap()
            .render(context! { first_name => first_name, new_email => new_email, code => code })
            .unwrap();
        let html = self
            .0
            .get_template(NAME_APPROVE_EMAIL_CHANGE_EMAIL_HTML)
            .unwrap()
            .render(context! { first_name => first_name, new_email => new_email, code => code })
            .unwrap();

        let subject = format!("[{APP_NAME}] Please approve your {APP_NAME} account email change.");

        EmailMessage::from_server(server_email, user_email, &subject, text, html)
    }

    /// Return password reset email
    pub fn password_reset(
        &self,
        server_email: &str,
        first_name: &str,
        user_email: &str,
        link: &str,
    ) -> ServerResult<EmailMessage> {
        let text = self
            .0
            .get_template(NAME_PASSWORD_RESET_EMAIL_TEXT)
            .unwrap()
            .render(context! { first_name => first_name, link => link })
            .unwrap();
        let html = self
            .0
            .get_template(NAME_PASSWORD_RESET_EMAIL_HTML)
            .unwrap()
            .render(context! { first_name => first_name, link => link })
            .unwrap();

        let subject = format!("[{APP_NAME}] Password reset.");

        EmailMessage::from_server(server_email, user_email, &subject, text, html)
    }

    /// Return verify new-email email
    pub fn verify_new_email(
        &self,
        server_email: &str,
        first_name: &str,
        new_email: &str,
        code: &str,
    ) -> ServerResult<EmailMessage> {
        let text = self
            .0
            .get_template(NAME_APPROVE_EMAIL_CHANGE_EMAIL_TEXT)
            .unwrap()
            .render(context! { first_name => first_name, new_email => new_email, code => code })
            .unwrap();
        let html = self
            .0
            .get_template(NAME_APPROVE_EMAIL_CHANGE_EMAIL_HTML)
            .unwrap()
            .render(context! { first_name => first_name, new_email => new_email, code => code })
            .unwrap();

        let subject = format!("[{APP_NAME}] Verify your new {APP_NAME} account email.");

        EmailMessage::from_server(server_email, new_email, &subject, text, html)
    }
}
