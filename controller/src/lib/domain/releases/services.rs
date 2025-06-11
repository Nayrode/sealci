use async_trait::async_trait;
use crate::domain::releases::entities::{CreateReleaseRequest, CreateReleaseResponse};

#[async_trait]
pub trait ReleaseAgentClient: Send + Sync {
    async fn release(&self, request: CreateReleaseRequest) -> Result<CreateReleaseResponse, Box<dyn std::error::Error>>;
}
