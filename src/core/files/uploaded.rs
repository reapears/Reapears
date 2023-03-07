//! Uploaded file impls

use axum::extract::multipart::Field;
use camino::{Utf8Path, Utf8PathBuf};
use uuid::Uuid;

use crate::{db, endpoint::EndpointRejection, error::ServerResult};

/// Raw file uploaded via `Multipart`
#[derive(Clone, Debug)]
pub struct UploadedFile {
    /// file unique id
    pub id: Uuid,
    /// file name without extension
    pub stem: String,
    /// file name extension
    pub ext: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub field_name: Option<String>,
}

impl UploadedFile {
    #[tracing::instrument(fields(file_name, content_type))]
    pub async fn try_from_field<'a>(field: Field<'a>) -> Result<Self, EndpointRejection> {
        let Some(file_name) = field.file_name() else{
            tracing::error!("Rejected: uploaded file has no name");
            return Err(EndpointRejection::BadRequest("File name required".into()));
        };
        tracing::Span::current().record("file_name", file_name);

        let Some(file_ext) = Utf8Path::new(file_name).extension().map(std::borrow::ToOwned::to_owned) else {
            tracing::error!("Rejected: uploaded file's name:{} has no extension", file_name);
            return Err(EndpointRejection::BadRequest(
                "File name has no extension".into(),
            ));
        };

        // Safe to unwrap here because file_ext extraction has passed (previous lines)
        let file_stem = Utf8Path::new(&file_name).file_stem().unwrap().to_owned();

        let Some(content_type) = field.content_type().map(std::borrow::ToOwned::to_owned) else{
            tracing::error!("Rejected: uploaded file has no content type");
            return Err(EndpointRejection::BadRequest(
                "Content type required".into(),
            ));
        };
        tracing::Span::current().record("content_type", &content_type);

        let field_name = field.name().map(std::borrow::ToOwned::to_owned);

        let content = match field.bytes().await {
            Ok(bytes) => bytes.to_vec(),
            Err(err) => {
                tracing::error!("Uploaded file bytes error: `{:?}`", err);
                return Err(EndpointRejection::BadRequest(err.to_string().into()));
            }
        };

        let file_id = db::model_id();

        Ok(Self {
            id: file_id,
            stem: file_stem,
            ext: file_ext,
            content,
            content_type,
            field_name,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn save(self, to: String) -> ServerResult<Utf8PathBuf> {
        let path = Utf8Path::new(&to).join(self.stem).join(self.ext);
        super::save_file(&path, &self.content).await
    }

    #[must_use]
    pub fn file_name(&self) -> String {
        format!("{}.{}", self.stem.clone(), self.ext.clone())
    }
}
