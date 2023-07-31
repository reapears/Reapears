//! `UploadedImage` impls

use std::{
    io::Cursor,
    path::{Path, PathBuf},
    sync::Arc,
};

use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use tokio::task::{self, JoinSet};

use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    error::{ServerError, ServerResult},
    types::ModelID,
};

use super::UploadedFile;

/// `DynamicImage` file decode from `UploadedFile`
#[derive(Debug, Clone)]
pub struct UploadedImage(Arc<InnerImage>);

impl UploadedImage {
    /// The unique file-name the image will be saved in.
    #[must_use]
    pub fn id(&self) -> ModelID {
        self.0.id
    }

    /// The original name of the image uploaded with.
    #[must_use]
    pub fn original_name(&self) -> &String {
        &self.0.file_name
    }

    /// Image content
    #[must_use]
    pub fn dynamic_image(&self) -> &DynamicImage {
        &self.0.image
    }

    /// The format the image was uploaded in.
    #[must_use]
    pub fn format(&self) -> ImageFormat {
        self.0.file_ext
    }

    /// The format the image was uploaded in.
    #[must_use]
    pub fn format_str(&self) -> &'static str {
        self.0.file_ext.extensions_str()[0]
    }

    /// Saves an image in the current format to the `upload dir`.
    pub async fn save<P>(self, upload_dir: P) -> ServerResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        task::spawn_blocking(move || self.0.save(upload_dir)).await?
    }

    /// Saves an image in the current format to the `upload dir`.
    pub async fn save_as<P>(self, ext: P, upload_dir: P) -> ServerResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        task::spawn_blocking(move || self.0.save_as(ext, upload_dir)).await?
    }

    /// Saves image in all supported output formats.
    pub async fn save_all<P>(self, upload_dir: P) -> ServerResult<Vec<PathBuf>>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let mut tasks = JoinSet::new();
        let upload_dir = upload_dir.as_ref();

        for ext in crate::IMAGE_OUTPUT_FORMATS {
            let upload_dir = upload_dir.to_owned();
            let img = self.clone();
            tasks.spawn_blocking(move || img.0.save_raw(ext, upload_dir));
        }

        // Wait for tasks to complete saving images
        let mut paths = Vec::new();
        while let Some(join_result) = tasks.join_next().await {
            match join_result {
                Ok(task_result) => match task_result {
                    Ok(path) => paths.push(path),
                    Err(err) => {
                        // Could not complete delete other saved files.
                        tokio::spawn(async move { super::delete_files(paths).await });
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
}

impl UploadedFile {
    /// Saves as an image to the file system in all server image output formats.
    pub async fn save_image<P>(self, upload_dir: P) -> EndpointResult<Vec<PathBuf>>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let paths = self.try_into_image().await?.save_all(upload_dir).await?;
        Ok(paths)
    }

    /// Saves as an image to the file system in it's original format.
    pub async fn save_image_original<P>(self, upload_dir: P) -> EndpointResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let path = self.try_into_image().await?.save(upload_dir).await?;
        Ok(path)
    }

    /// Try parse `UploadedFile` into `UploadedImage`
    pub async fn try_into_image(self) -> EndpointResult<UploadedImage> {
        match task::spawn_blocking(move || {
            let ext = self.file_ext.clone();
            // fails if the image format is not supported
            if !crate::SUPPORTED_UPLOAD_IMAGE_FORMATS.contains(&ext.as_ref()) {
                tracing::error!("Unsupported image form: {ext:?}");
                return Err(ServerError::bad_request(format!(
                    "Unsupported image format: {ext:?}. Supported formats: jpg, png."
                )));
            }
            // Safety: the image format is supported as it passed the first check
            let uploaded_format = ImageFormat::from_extension(&ext).unwrap();

            let reader = ImageReader::new(Cursor::new(&self.content)).with_guessed_format()?;
            let ext = reader.format().unwrap_or(uploaded_format);

            let image = reader.decode()?;

            Ok(UploadedImage(Arc::new(InnerImage {
                id: self.id,
                file_name: self.file_stem,
                file_ext: ext,
                image,
            })))
        })
        .await
        {
            Ok(decode_result) => Ok(decode_result?),
            Err(_join_err) => Err(EndpointRejection::internal_server_error()),
        }
    }
}

/// Image file decode from `UploadedFile`
#[derive(Clone, Debug)]
struct InnerImage {
    id: ModelID,
    file_name: String,
    file_ext: ImageFormat,
    image: DynamicImage,
}

#[allow(clippy::needless_pass_by_value)]
impl InnerImage {
    /// Saves an image in current format to the `upload dir`.
    fn save<P>(&self, upload_dir: P) -> ServerResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        self.save_raw(self.file_ext, upload_dir)
    }

    /// Saves an image in given format(`ext`) to the `upload dir`.
    ///
    ///  Like `save_raw` but flexible on `ext` type.
    fn save_as<P>(&self, ext: P, upload_dir: P) -> ServerResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let ext = ImageFormat::from_extension(ext.as_ref()).ok_or_else(|| {
            tracing::error!("Image error, unsupported image format: {:?}", ext.as_ref());
            ServerError::new(format!("Unsupported image format: {:?}", ext.as_ref()))
        })?;

        self.save_raw(ext, upload_dir)
    }

    /// Saves an image in given format(`ext`) to the `upload dir`.
    fn save_raw<P>(&self, fmt: ImageFormat, upload_dir: P) -> ServerResult<PathBuf>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let path = upload_dir
            .as_ref()
            .join(self.id.to_string())
            .with_extension(fmt.extensions_str()[0]);

        self.image.save(&path).map_err(|err| {
            tracing::error!("Image error, failed to save an image: {}", err);
            <image::ImageError as std::convert::Into<ServerError>>::into(err)
        })?;

        Ok(path)
    }
}
