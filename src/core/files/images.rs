//! Image file impls

use std::{
    path::{Path, PathBuf},
    {fmt, io::Cursor},
};

use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use tokio::task::{self, JoinSet};

use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    error::{ServerError, ServerResult},
    types::ModelID,
};

use super::UploadedFile;

const SUPPORTED_UPLOAD_IMAGE_FORMATS: [&str; 4] = ["jpeg", "jpg", "png", "webp"];
const SUPPORTED_IMAGE_OUTPUT_FORMAT: [SupportedImageOutputFormat; 2] = [
    SupportedImageOutputFormat::Jpeg,
    SupportedImageOutputFormat::WebP,
];

/// Image file decode from `UploadedFile`
#[derive(Clone, Debug)]
pub struct ImageFile {
    pub id: ModelID,
    /// File stem
    pub stem: String,
    pub image: DynamicImage,
    /// File extension
    pub format: ImageFormat,
}

impl ImageFile {
    /// Save the image using the original format
    #[tracing::instrument(skip(self, save_to))]
    pub async fn save_original<T>(self, save_to: T) -> ServerResult<PathBuf>
    where
        T: fmt::Display + Send + 'static,
    {
        match tokio::spawn(async move { self.__save(save_to) }).await {
            Ok(save_result) => save_result,
            Err(join_err) => Err(ServerError::internal(Box::new(join_err))),
        }
    }

    /// Save a file using a given image-format
    #[tracing::instrument(skip(self, save_to))]
    pub async fn save<T>(
        self,
        save_to: T,
        format: SupportedImageOutputFormat,
    ) -> ServerResult<PathBuf>
    where
        T: fmt::Display + Send + 'static,
    {
        let mut image = self;
        image.change_format(format)?;

        image.save_original(save_to).await
    }

    /// Save image using all supported output formats
    #[tracing::instrument(skip(self, save_to))]
    pub async fn save_all_format<T>(self, save_to: T) -> ServerResult<Vec<PathBuf>>
    where
        T: fmt::Display + Send + 'static,
    {
        let mut tasks = JoinSet::new();
        for format in SUPPORTED_IMAGE_OUTPUT_FORMAT {
            let mut img = self.clone();
            let save_to = save_to.to_string().clone();
            img.change_format(format)?;

            tasks.spawn_blocking(move || img.__save(save_to));
        }

        let mut paths = Vec::new();
        // Wait for tasks to complete saving images
        while let Some(join_result) = tasks.join_next().await {
            match join_result {
                Ok(task_result) => match task_result {
                    Ok(path) => paths.push(path),
                    Err(err) => {
                        //    // Could not complete delete other files
                        //    tokio::spawn(super::delete_files(paths).await)

                        return Err(err);
                    }
                },
                Err(err) => {
                    tracing::error!("Join handler error: {}", err);
                    return Err(ServerError::internal(Box::new(err)));
                }
            }
        }
        Ok(paths)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __save<T>(&self, save_to: T) -> ServerResult<PathBuf>
    where
        T: fmt::Display + Send + 'static,
    {
        let format = self.format.extensions_str()[0];
        let path = Path::new(&save_to.to_string()).join(format!("{}.{}", self.id, format));
        tracing::info!("{}", &format!("Saving an image at: {path:?}"));
        match self.image.save(&path) {
            Ok(()) => {
                tracing::info!("{}", &format!("Completed saving an image at: {path:?}"));
                Ok(path)
            }
            Err(err) => {
                tracing::error!("Failed to save an image, err:{}", err);
                Err(err.into())
            }
        }
    }

    /// Change image format
    #[allow(clippy::needless_pass_by_value)]
    fn change_format(&mut self, format: SupportedImageOutputFormat) -> ServerResult<()> {
        if let Some(new_format) = ImageFormat::from_extension(format.to_string()) {
            self.format = new_format;
            Ok(())
        } else {
            tracing::error!("{}", format!("Failed to change image format to: {format}"));
            Err(ServerError::new(format!(
                "Unsupported image format: {format}"
            )))
        }
    }
}

impl UploadedFile {
    /// Try parse `UploadedFile` into `ImageFile`
    ///
    /// # Errors
    ///
    /// Return an error if the image format is not supported,
    pub async fn try_into_image(self) -> ServerResult<ImageFile> {
        match task::spawn_blocking(move || {
            let ext = self.ext.clone();
            // fails if the image format is not supported
            if !SUPPORTED_UPLOAD_IMAGE_FORMATS.contains(&ext.as_ref()) {
                tracing::error!("Unsupported image form: {ext}");
                return Err(ServerError::bad_request(format!(
                    "Invalid/Unsupported image format: {ext}. Supported formats: jpg, png."
                )));
            }
            // Safety: the image format is supported as it passed the first check
            let uploaded_format = ImageFormat::from_extension(&ext).unwrap();

            let reader = ImageReader::new(Cursor::new(&self.content)).with_guessed_format()?;
            let format = reader.format().unwrap_or(uploaded_format);

            let image = reader.decode()?;

            Ok(ImageFile {
                stem: self.stem,
                id: self.id,
                format,
                image,
            })
        })
        .await
        {
            Ok(decode_result) => decode_result,
            Err(join_err) => Err(ServerError::internal(Box::new(join_err))),
        }
    }
}

/// Save an image to the file system
///
/// # Error
///
/// Return an image or io error
pub async fn save_image<T>(file: UploadedFile, upload_dir: T) -> EndpointResult<Vec<PathBuf>>
where
    T: fmt::Display + Send + 'static,
{
    // Decode uploaded file into an image
    let saved_to = file
        .try_into_image()
        .await
        .map_err(|err| EndpointRejection::BadRequest(err.to_string().into()))?
        // Save image in jpg and webp formats
        .save_all_format(upload_dir)
        .await
        .map_err(|err| EndpointRejection::InternalServerError(err.into()))?;

    Ok(saved_to)
}

/// Supported image formats on the server
#[derive(Debug, Clone)]
pub enum SupportedImageOutputFormat {
    Jpeg,
    Png,
    WebP,
}

impl fmt::Display for SupportedImageOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jpeg => f.write_str("jpg"),
            Self::Png => f.write_str("png"),
            Self::WebP => f.write_str("webp"),
        }
    }
}
