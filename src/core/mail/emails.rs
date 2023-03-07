//! Emails impls

mod account_confirmation;
mod email_change;
mod password_reset;

pub use account_confirmation::account_confirmation_email;
pub use email_change::confirmation_code_email;
pub use password_reset::password_reset_email;
