use git2::AutotagOption;
use git2::FetchOptions;
use std::path::PathBuf;
use tracing::debug;
use tracing::error;

use tonic::async_trait;

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait GitClient: Clone + Send + Sync {
    // revision is a string of the form "v1.2.3" that is the release name, it returns the path to
    // the folder that contains the codebase
    async fn download_release(
        &self,
        repository_url: String,
        revision: String,
    ) -> Result<PathBuf, ReleaseAgentError>;
    fn clean_release(&self, path: PathBuf) -> Result<(), ReleaseAgentError>;
}

#[derive(Debug, Clone)]
pub struct Git2Client {
    pub path: PathBuf,
}

impl Git2Client {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait]
impl GitClient for Git2Client {
    async fn download_release(
        &self,
        repository_url: String,
        revision: String,
    ) -> Result<PathBuf, ReleaseAgentError> {
        let repository_name = repository_url.split("/").last().unwrap();
        let folder_name = format!("{}-{}", repository_name, revision);
        let folder_path = self.path.join(folder_name.clone());
        std::fs::create_dir_all(&folder_path).map_err(|e| {
            error!("Error creating folder: {}", e);
            ReleaseAgentError::GitRepositoryNotFound
        })?;
        debug!("Cloning repository '{repository_url}' to '{folder_name}'");
        let mut builder = git2::build::RepoBuilder::new();
        let mut fetch_options = FetchOptions::new();
        let repo = builder
            .clone(repository_url.as_str(), folder_path.as_path())
            .map_err(|e| {
                error!("Error cloning repository: {}", e);
                ReleaseAgentError::GitRepositoryNotFound
            })?;
        fetch_options.download_tags(AutotagOption::All);
        builder.fetch_options(fetch_options);
        repo.set_head(format!("refs/tags/{}", revision).as_str())
            .map_err(|e| {
                error!("Error setting head: {}", e);
                ReleaseAgentError::GitTagNotFound
            })?;

        Ok(folder_path)
    }

    fn clean_release(&self, path: PathBuf) -> Result<(), ReleaseAgentError> {
        std::fs::remove_dir_all(path.clone()).map_err(|e| {
            error!("Error removing folder: {} at {}", e, path.display());
            ReleaseAgentError::GitRepositoryNotFound
        })?;
        Ok(())
    }
}
