//! File impls

use camino::{Utf8Path, Utf8PathBuf};
use tokio::{fs, task::JoinSet};

use crate::error::{ServerError, ServerResult};

mod handler;
mod images;
mod uploaded;

pub use handler::accept_uploads;
pub use images::{save_image, ImageFile};
pub use uploaded::UploadedFile;

/// Image formats stored on the server
pub const IMAGE_FORMATS: [&str; 2] = ["jpg", "webp"];

/// Image maximum size allowed on the server
pub const IMAGE_MAX_SIZE: usize = 20 * 1024 * 1024; // 20 * 1024 * 1024 /* 20mb */
/// Saves file on the filesystem
#[tracing::instrument(skip(content))]
pub async fn save_file(path: &Utf8Path, content: &[u8]) -> ServerResult<Utf8PathBuf> {
    fs::write(&path, content).await?;
    Ok(path.to_owned())
}

/// Deletes file from the filesystem
#[tracing::instrument]
pub async fn delete_file(path: &Utf8Path) -> ServerResult<()> {
    match fs::remove_file(&path).await {
        Ok(()) => Ok(()),
        Err(err) => {
            tracing::error!("Error occurred while deleting file: {path}");
            Err(err.into())
        }
    }
}

/// Deletes a list of files
#[tracing::instrument]
pub async fn delete_files(paths: Vec<Utf8PathBuf>) -> ServerResult<()> {
    let mut tasks = JoinSet::new();
    for path in paths {
        tasks
            .build_task()
            .name(&format!("Deleting file: `{path}`."))
            .spawn(async move { delete_file(path.as_path()).await })?;
    }
    // Wait for tasks to complete deleting
    while let Some(join_result) = tasks.join_next().await {
        match join_result {
            Ok(task_result) => match task_result {
                Ok(()) => continue,
                Err(err) => {
                    tracing::error!("Io error: {}", err);
                    // Should continue deleting the remaining ones
                }
            },
            Err(err) => {
                tracing::error!("Join handler error: {}", err);
                return Err(ServerError::internal(Box::new(err)));
            }
        }
    }
    Ok(())
}

/// Deletes directory and its content
#[tracing::instrument]
pub async fn delete_dir(path: &Utf8Path) -> ServerResult<()> {
    match fs::remove_dir_all(&path).await {
        Ok(()) => Ok(()),
        Err(err) => {
            tracing::error!("Error occurred while deleting directory and its content: {path}");
            Err(err.into())
        }
    }
}

/// Get `file_name` where the extension is `jpg`
///
/// # Errors
///
/// Return an error if jpg file path could not be found
///
/// # Panics
///
/// May Panic if file extension could not be extracted
#[allow(dead_code)]
pub fn get_jpg_path(paths: Vec<Utf8PathBuf>) -> ServerResult<String> {
    paths
        .into_iter()
        .filter(|path| path.extension().is_some())
        .find(|file| file.extension().unwrap().to_lowercase() == "jpg")
        .filter(|path| path.file_name().is_some())
        .map(|path| path.file_name().unwrap().to_owned())
        .ok_or_else(|| ServerError::new("jpg file_name could not be found."))
}

/// Get all image formats paths saved on the server
#[must_use]
pub fn saved_paths(upload_dir: &str, file: &str) -> Vec<Utf8PathBuf> {
    let path = Utf8PathBuf::from(file);
    let stem = path.file_stem().unwrap_or(file);
    IMAGE_FORMATS
        .into_iter()
        .map(|ext| Utf8PathBuf::from(format!("{upload_dir}/{stem}.{ext}")))
        .collect()
}
