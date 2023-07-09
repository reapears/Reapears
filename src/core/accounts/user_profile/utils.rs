//! User profile helpers impls

use crate::{error::ServerResult, files, settings};

///  Delete user profile-photo fom the file system
///
/// # Errors
///
/// Return io error
pub async fn delete_user_photo(file_name: &str) -> ServerResult<()> {
    let paths = files::saved_paths(settings::USER_UPLOAD_DIR, file_name);
    files::delete_files(paths).await
}
