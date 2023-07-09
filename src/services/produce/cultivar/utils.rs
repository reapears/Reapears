//! Cultivar helpers impls

use crate::{error::ServerResult, files, settings::CULTIVAR_UPLOAD_DIR};

///  Delete cultivar images fom the file system
///
/// # Errors
///
/// Return io errors
pub async fn delete_cultivar_photo(file_name: &str) -> ServerResult<()> {
    let paths = files::saved_paths(CULTIVAR_UPLOAD_DIR, file_name);
    files::delete_files(paths).await
}
