use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: i64,
    pub repo_url: String,
    pub revision: String,
    pub path: String,
    pub public_key: String,
    pub fingerprint: String,
}

impl Release {
    pub fn new(
        id: i64,
        repo_url: String,
        revision: String,
        path: String,
        public_key: String,
        fingerprint: String,
    ) -> Release {
        Release {
            id,
            repo_url,
            revision,
            path,
            public_key,
            fingerprint,
        }
    }
}

#[derive(Debug, Error)]
pub enum ReleaseError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Release not found")]
    NotFound,
    #[error("Internal error")]
    InternalError,
    #[error("Release agent error")]
    ReleaseAgentError,
}

#[derive(Debug, Clone)]
pub struct CreateReleaseRequest {
    pub repo_url: String,
    pub revision: String,
}

#[derive(Debug, Clone)]
pub struct CreateReleaseResponse {
    pub status: ReleaseStatus,
    pub release_id: String,
    pub public_key: Option<PublicKey>,
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key_data: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReleaseStatus {
    SUCCESS = 0,
    FAILURE = 1,
}
