//! User profile helpers impls

use crate::{error::ServerResult, files, settings};

///  Delete user profile-photo fom the file system
///
/// # Errors
///
/// Return io error
pub async fn delete_user_photo(file_name: &str) -> ServerResult<()> {
    let dir = settings::user_uploads_dir();
    let paths = files::saved_paths(&dir, file_name);
    files::delete_files(paths).await
}
