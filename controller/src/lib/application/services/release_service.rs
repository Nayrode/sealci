use async_trait::async_trait;
use std::{path::PathBuf, sync::Arc};
use tracing::info;

use crate::{
    application::ports::release_service::ReleaseService,
    domain::releases::{
        entities::{CreateReleaseRequest, Release, ReleaseError, ReleaseStatus},
        ports::ReleaseRepository,
        services::ReleaseAgentClient,
    },
};

pub struct ReleaseServiceImpl<R, P>
where
    R: ReleaseAgentClient + Send + Sync,
    P: ReleaseRepository + Send + Sync,
{
    release_agent_client: Arc<R>,
    release_repository: Arc<P>,
}

impl<R, P> ReleaseServiceImpl<R, P>
where
    R: ReleaseAgentClient + Send + Sync,
    P: ReleaseRepository + Send + Sync,
{
    pub fn new(release_agent_client: Arc<R>, release_repository: Arc<P>) -> Self {
        Self {
            release_agent_client,
            release_repository,
        }
    }
}

#[async_trait]
impl<R, P> ReleaseService for ReleaseServiceImpl<R, P>
where
    R: ReleaseAgentClient + Send + Sync,
    P: ReleaseRepository + Send + Sync,
{
    async fn create_release(&self, repo_url: &str, revision: &str) -> Result<(), ReleaseError> {
        let request = CreateReleaseRequest {
            repo_url: repo_url.to_string(),
            revision: revision.to_string(),
        };
        let client = self.release_agent_client.clone();
        let release_answer = client
            .release(request)
            .await
            .map_err(|_| ReleaseError::InternalError)?;
        if release_answer.status == ReleaseStatus::FAILURE {
            return Err(ReleaseError::ReleaseAgentError);
        }
        let public_key = release_answer.public_key.unwrap().clone();
        info!("pk : {}", public_key.key_data);
        let _ = self
            .release_repository
            .create_release(
                repo_url.to_string(),
                revision.to_string(),
                release_answer.release_id,
                public_key.key_data,
                public_key.fingerprint,
            )
            .await?;
        Ok(())
    }

    async fn list_releases(&self, repo_url: &str) -> Result<Vec<Release>, ReleaseError> {
        self.release_repository
            .list_releases(repo_url.to_string())
            .await
    }
}
