use async_trait::async_trait;

use super::entities::{Release, ReleaseError};

#[async_trait]
pub trait ReleaseRepository: Send + Sync {
    async fn create_release(
        &self,
        repo_url: String,
        revision: String,
        path: String,
        public_key: String,
        fingerprint: String,
    ) -> Result<Release, ReleaseError>;
    async fn list_releases(&self, repo_url: String) -> Result<Vec<Release>, ReleaseError>;
}
