//! Image file impls

use camino::{Utf8Path, Utf8PathBuf};
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use tokio::task::{self, JoinSet};
use uuid::Uuid;

use std::{fmt, io::Cursor};

use super::UploadedFile;
use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    error::{ServerError, ServerResult},
};

const SUPPORTED_UPLOAD_IMAGE_FORMATS: [&str; 4] = ["jpeg", "jpg", "png", "webp"];
const SUPPORTED_IMAGE_OUTPUT_FORMAT: [SupportedImageOutputFormat; 2] = [
    SupportedImageOutputFormat::Jpeg,
    SupportedImageOutputFormat::WebP,
];

/// Image file decode from `UploadedFile`
#[derive(Clone, Debug)]
pub struct ImageFile {
    pub id: Uuid,
    pub stem: String,
    pub image: DynamicImage,
    pub format: ImageFormat,
}

impl ImageFile {
    /// Save the image using the original format
    #[tracing::instrument(skip(self, save_to))]
    pub async fn save_original<T>(self, save_to: T) -> ServerResult<Utf8PathBuf>
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
    ) -> ServerResult<Utf8PathBuf>
    where
        T: fmt::Display + Send + 'static,
    {
        let mut image = self;
        image.change_format(format)?;

        image.save_original(save_to).await
    }

    /// Save image using all supported output formats
    #[tracing::instrument(skip(self, save_to))]
    pub async fn save_all_format<T>(self, save_to: T) -> ServerResult<Vec<Utf8PathBuf>>
    where
        T: fmt::Display + Send + 'static,
    {
        let mut tasks = JoinSet::new();
        for format in SUPPORTED_IMAGE_OUTPUT_FORMAT {
            let mut image = self.clone();
            let save_to = save_to.to_string().clone();
            image.change_format(format)?;

            tasks
                .build_task()
                .spawn(async move { image.__save(save_to) })?;
        }

        let mut paths = Vec::new();
        // Wait for tasks to complete deleting
        while let Some(join_result) = tasks.join_next().await {
            match join_result {
                Ok(task_result) => match task_result {
                    Ok(path) => paths.push(path),
                    Err(err) => return Err(err),
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
    fn __save<T>(&self, save_to: T) -> ServerResult<Utf8PathBuf>
    where
        T: fmt::Display + Send + 'static,
    {
        let format = self.format.extensions_str()[0];
        let path = Utf8Path::new(&save_to.to_string()).join(format!("{}.{}", self.id, format));
        tracing::info!("{}", &format!("Saving an image at: {path}"));
        match self.image.save(&path) {
            Ok(()) => {
                tracing::info!("{}", &format!("Completed saving an image at: {path}"));
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
    /// Try parse `Self` into `ImageFile`
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
                return Err(ServerError::new(format!(
                    "Unsupported image format: {ext}. Supported formats: jpg, png."
                )));
            }
            let uploaded_format = ImageFormat::from_extension(&ext)
                .ok_or_else(|| ServerError::new(format!("Unsupported image format: {ext}")))?;

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
pub async fn save_image<T>(file: UploadedFile, upload_dir: T) -> EndpointResult<Vec<Utf8PathBuf>>
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

// /// Supported image formats on the server
// #[derive(Debug, Clone)]
// pub enum SupportedImageFormat {
//     Jpeg,
//     Png,
// }

// impl SupportedImageFormat {
//     pub fn from_path(path: &str) -> ServerResult<Self> {
//         match Utf8Path::new(path).extension() {
//             Some(ext) => Self::from_str(ext),
//             None => {
//                 tracing::error!("Could not extract image format from path: {path}");
//                 Err(
//                     ServerError::new(format!("Failed to extract image format from : `{path}`"))
//                         .into(),
//                 )
//             }
//         }
//     }
// }

// /// Parse SupportedImageFormat from file extension
// impl FromStr for SupportedImageFormat {
//     type Err = ServerError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match ImageFormat::from_extension(s) {
//             Some(format) => format.try_into(),
//             None => Err(ServerError::new(format!("Unsupported image format: {s}")).into()),
//         }
//     }
// }

// impl fmt::Display for SupportedImageFormat {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Jpeg => f.write_str("jpg"),
//             Self::Png => f.write_str("png"),
//         }
//     }
// }

// impl From<SupportedImageFormat> for ImageFormat {
//     fn from(value: SupportedImageFormat) -> Self {
//         match value {
//             SupportedImageFormat::Jpeg => ImageFormat::Jpeg,
//             SupportedImageFormat::Png => ImageFormat::Png,
//         }
//     }
// }

// impl TryFrom<ImageFormat> for SupportedImageFormat {
//     type Error = ServerError;
//     fn try_from(value: ImageFormat) -> Result<Self, Self::Error> {
//         match value {
//             ImageFormat::Jpeg => Ok(Self::Jpeg),
//             ImageFormat::Png => Ok(Self::Png),
//             _ => Err(ServerError::new(format!("Unsupported image format: {value:?}")).into()),
//         }
//     }
// }
