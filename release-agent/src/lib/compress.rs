use std::{fs::File, path::PathBuf};
use tonic::async_trait;
use tracing::{error, info};

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait CompressClient: Clone + Send + Sync {
    // path is a string containing the path to the folder that contains the codebase to be compressed
    // the File object returned contains the compressed codebase
    async fn compress(&self, path: PathBuf) -> Result<(File, PathBuf), ReleaseAgentError>;

    fn clean_compressed(&self, path: PathBuf) -> Result<(), ReleaseAgentError>;
}

#[derive(Debug, Clone)]
pub struct Flate2Client {}

impl Flate2Client {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CompressClient for Flate2Client {
    async fn compress(&self, path: PathBuf) -> Result<(File, PathBuf), ReleaseAgentError> {
        let file_name = path.with_extension("tar.gz");
        let file =
            File::create(file_name.clone()).map_err(|_| ReleaseAgentError::CompressionError)?;
        let encoder = flate2::write::GzEncoder::new(
            file.try_clone()
                .map_err(|_| ReleaseAgentError::CompressionError)?,
            flate2::Compression::default(),
        );
        let mut tar = tar::Builder::new(encoder);
        tar.append_dir_all(".", path)
            .map_err(|_| ReleaseAgentError::CompressionError)?;
        tar.finish()
            .map_err(|_| ReleaseAgentError::CompressionError)?;
        info!("Compressed codebase to {}", file_name.display());
        Ok((file, file_name))
    }

    fn clean_compressed(&self, path: PathBuf) -> Result<(), ReleaseAgentError> {
        std::fs::remove_file(path).map_err(|e| {
            error!("Error removing file: {}", e);
            ReleaseAgentError::CompressionError
        })?;
        Ok(())
    }
}
