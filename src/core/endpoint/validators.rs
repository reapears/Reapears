//! User input validators impls

use super::{EndpointRejection, EndpointResult};
use crate::types::ModelID;

/// Validate string user inputs
pub trait ValidateString {
    /// Validates the string is valid `ModelId`
    fn validate_id(&self, err_msg: &'static str) -> EndpointResult<()>;

    /// Validates the string length is within min and max bounds
    fn validate_len(&self, min: u8, max: u16, err_msg: &'static str) -> EndpointResult<()>;

    /// Validates the string is a valid email address
    fn validate_email(&self) -> EndpointResult<()>;

    /// Validates string is valid phone number
    fn validate_phone(&self) -> EndpointResult<String>;
}

impl ValidateString for str {
    /// Validates the string length is within min and max bounds
    fn validate_len(&self, min: u8, max: u16, err_msg: &'static str) -> EndpointResult<()> {
        let me = self.trim();
        let len = me.len();
        if len < min as usize {
            tracing::error!("Validation length error: {}", err_msg);
            return Err(EndpointRejection::BadRequest(err_msg.into()));
        }
        if len > max as usize {
            tracing::error!("Validation length error: {}", err_msg);
            return Err(EndpointRejection::BadRequest(err_msg.into()));
        }
        Ok(())
    }

    /// Validates the string is valid `ModelId`
    fn validate_id(&self, err_msg: &'static str) -> EndpointResult<()> {
        if ModelID::try_from(self).is_ok() {
            Ok(())
        } else {
            tracing::error!("Validation id error: {}", err_msg);
            Err(EndpointRejection::BadRequest(err_msg.into()))
        }
    }

    /// Validates the string is a valid email address
    fn validate_email(&self) -> EndpointResult<()> {
        if self.parse::<lettre::Address>().is_ok() {
            Ok(())
        } else {
            tracing::error!("Validation email error: invalid email address.");
            Err(EndpointRejection::BadRequest(
                "Invalid email address".into(),
            ))
        }
    }

    /// Validates string is valid Namibian phone-number
    fn validate_phone(&self) -> EndpointResult<String> {
        match phonenumber::parse(Some(phonenumber::country::Id::NA), self) {
            Ok(phone) if phonenumber::is_valid(&phone) => Ok(phone.to_string()),
            Ok(_) | Err(_) => {
                tracing::error!("Validation phone error: invalid phone number.");
                Err(EndpointRejection::BadRequest("Invalid phone number".into()))
            }
        }
    }
}

// ====== String Transformation =====

/// String extension trait
pub trait TransformString {
    /// Transforms a string in title case
    fn to_titlecase(&self) -> String;

    /// Trim string whitespace
    fn clean(&self) -> String;
}
impl TransformString for str {
    /// Transforms a string in title case
    ///
    /// Where every word starts with a capital letter
    fn to_titlecase(&self) -> String {
        let s = self.trim();
        let words: Vec<_> = s
            .split_whitespace()
            .map(|w| {
                let w = w.to_ascii_lowercase();
                let mut chars = w.chars();
                let mut first_letter = chars
                    .next()
                    .expect("Failed to get the first letter of the word");
                first_letter.make_ascii_uppercase();
                let others: String = chars.collect();

                format!("{first_letter}{others}")
            })
            .collect();
        words.join(" ")
    }

    /// trim whitespace
    fn clean(&self) -> String {
        self.trim().to_owned()
    }
}

// // ===== Utilities impls =====

// /// Wait for all validation tasks to complete
// ///
// /// # Errors
// ///
// /// Return an error and abort all running tasks, if one the task failed to validate
// pub async fn join_validation_tasks(
//     mut tasks: JoinSet<EndpointResult<()>>,
//     task_handlers: &[AbortHandle],
// ) -> EndpointResult<()> {
//     while let Some(res) = tasks.join_next().await {
//         match res {
//             Ok(task_res) => match task_res {
//                 Ok(()) => {}
//                 Err(err) => {
//                     // Cancel all unfinished tasks
//                     for handler in task_handlers {
//                         if !handler.is_finished() {
//                             handler.abort();
//                         }
//                     }
//                     tracing::error!("Validation error: {}", err);
//                     return Err(EndpointRejection::BadRequest(err.to_string().into()));
//                 }
//             },
//             Err(err) => {
//                 tracing::error!("Server fault error: {}", err);
//                 return Err(EndpointRejection::internal_server_error());
//             }
//         }
//     }
//     Ok(())
// }
