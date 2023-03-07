//! Cultivar helpers impls

use crate::{error::ServerResult, files, settings};

///  Delete cultivar images fom the file system
///
/// # Errors
///
/// Return io errors
pub async fn delete_cultivar_photo(file_name: &str) -> ServerResult<()> {
    let dir = settings::cultivar_uploads_dir();
    let paths = files::saved_paths(&dir, file_name);
    files::delete_files(paths).await
}
