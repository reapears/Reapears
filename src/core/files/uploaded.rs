//! Uploaded file impls

use std::path::{Path, PathBuf};

use axum::extract::multipart::Field;

use crate::{endpoint::EndpointRejection, error::ServerResult, types::ModelID};

/// Raw file uploaded via `Multipart`
#[derive(Clone, Debug)]
pub struct UploadedFile {
    pub id: ModelID,
    pub file_stem: String,
    pub file_ext: String,
    pub content: bytes::Bytes,
    pub content_type: String,
    pub field_name: Option<String>,
}

impl UploadedFile {
    /// Parses `Multipart field` into an `UploadedFile`
    #[tracing::instrument(fields(file_name, content_type))]
    pub async fn try_from_field<'a>(field: Field<'a>) -> Result<Self, EndpointRejection> {
        let Some(file_name) = field.file_name() else {
            tracing::error!("Rejected: uploaded file has no name");
            return Err(EndpointRejection::BadRequest("File name required".into()));
        };
        tracing::Span::current().record("file_name", file_name);

        let Some(file_ext) = Path::new(file_name)
            .extension()
            .map(|ext| ext.to_string_lossy().to_string())
        else {
            tracing::error!(
                "Rejected: uploaded file's name:{} has no extension",
                file_name
            );
            return Err(EndpointRejection::BadRequest(
                "File name has no extension".into(),
            ));
        };

        // Safe to unwrap here because file_ext extraction has passed (previous lines)
        let file_stem = Path::new(&file_name)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let Some(content_type) = field.content_type().map(std::borrow::ToOwned::to_owned) else {
            tracing::error!("Rejected: uploaded file has no content type");
            return Err(EndpointRejection::BadRequest(
                "Content type required".into(),
            ));
        };
        tracing::Span::current().record("content_type", &content_type);

        let field_name = field.name().map(std::borrow::ToOwned::to_owned);

        let content = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(err) => {
                tracing::error!("Uploaded file bytes error: `{:?}`", err);
                return Err(EndpointRejection::BadRequest(err.to_string().into()));
            }
        };

        Ok(Self {
            id: ModelID::new(),
            file_stem,
            file_ext,
            content,
            content_type,
            field_name,
        })
    }

    /// Saves an uploaded file to the file system
    pub async fn save_uploaded<P>(self, upload_dir: P) -> ServerResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let path = upload_dir
            .as_ref()
            .join(self.file_stem)
            .with_extension(self.file_ext);
        super::save_file(&path, &self.content).await
    }
}
