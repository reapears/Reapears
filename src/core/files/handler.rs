//! User upload file handler impls

use axum::extract::Multipart;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{endpoint::EndpointRejection, files::UploadedFile};

///Accept file uploads via `multipart/form-data`
#[must_use]
pub fn accept_uploads(multipart: Multipart, file_count: u8) -> (UploadHandler, Uploads) {
    let (sender, receiver) = mpsc::channel(file_count.into());
    let handler = UploadHandler {
        multipart,
        file_count,
        sender,
    };
    let uploads = Uploads { receiver };

    (handler, uploads)
}

/// Accept file uploads via multipart/form
///
/// call `accept method` to start accepting files
#[derive(Debug)]
pub struct UploadHandler {
    pub multipart: Multipart,
    pub file_count: u8,
    pub sender: Sender<UploadedFile>,
}

impl UploadHandler {
    /// Accepts file uploads
    /// from the client and send them to [Uploads] via channels
    #[tracing::instrument(skip(self))]
    pub async fn accept(mut self) -> Result<(), EndpointRejection> {
        let mut received_file_count = 0;
        while let Some(field) = self
            .multipart
            .next_field()
            .await
            .map_err(|err| EndpointRejection::BadRequest(err.to_string().into()))?
        {
            // Only accept files until the max number of files given
            if received_file_count == self.file_count {
                return Ok(());
            }

            let file = UploadedFile::try_from_field(field).await?;

            if matches!(self.sender.send(file).await, Ok(())) {
                // Update received file count
                received_file_count += 1;
            } else {
                tracing::error!(
                    "Could not finish sending files: receiver dropped while sending files"
                );
                return Err(EndpointRejection::internal_server_error());
            }
        }
        Ok(())
    }
}

/// Receive uploaded files from `UploadHandler`
#[derive(Debug)]
pub struct Uploads {
    pub receiver: Receiver<UploadedFile>,
}

impl Uploads {
    pub async fn files(&mut self) -> Option<UploadedFile> {
        self.receiver.recv().await
    }
}
