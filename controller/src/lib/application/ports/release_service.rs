use crate::domain::releases::entities::{Release, ReleaseError};
use async_trait::async_trait;

#[async_trait]
pub trait ReleaseService: Send + Sync {
    async fn create_release(&self, repo_url: &str, revision: &str) -> Result<(), ReleaseError>;
    async fn list_releases(&self, repo_url: &str) -> Result<Vec<Release>, ReleaseError>;
    async fn get_key(&self, fingerprint: &str) -> Result<String, ReleaseError>;
}
