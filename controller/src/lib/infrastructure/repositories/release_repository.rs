use tonic::async_trait;
use std::sync::Arc;
use tracing::info;

use crate::{
    domain::releases::{
        entities::{Release, ReleaseError},
        ports::ReleaseRepository,
    },
    infrastructure::db::postgres::Postgres,
};

pub struct PostgresReleaseRepository {
    pub postgres: Arc<Postgres>,
}

impl PostgresReleaseRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

#[async_trait]
impl ReleaseRepository for PostgresReleaseRepository {
    async fn create_release(
        &self,
        repo_url: String,
        revision: String,
        path: String,
        public_key: String,
        fingerprint: String,
    ) -> Result<Release, ReleaseError> {
        info!("inserting {}", repo_url);
        let row = sqlx::query!(
            "INSERT INTO releases (repo_url, revision, path, public_key, fingerprint) VALUES ($1, $2, $3, $4, $5)
             RETURNING id, repo_url, revision, path, public_key, fingerprint",
            repo_url,
            revision,
            path,
            public_key,
            fingerprint
        )
        .fetch_one(&self.postgres.get_pool())
        .await
        .map_err(ReleaseError::DatabaseError)?;

        Ok(Release {
            id: row.id,
            repo_url: row.repo_url,
            revision: row.revision,
            path: row.path,
            public_key: row.public_key,
            fingerprint: row.fingerprint,
        })
    }

    async fn list_releases(&self, repo_url: String) -> Result<Vec<Release>, ReleaseError> {
        let rows = sqlx::query!(
            "SELECT id, repo_url, revision, path, public_key, fingerprint FROM releases WHERE repo_url = $1",
            repo_url
        )
        .fetch_all(&self.postgres.get_pool())
        .await
        .map_err(ReleaseError::DatabaseError)?;

        let releases = rows
            .into_iter()
            .map(|row| Release {
                id: row.id,
                repo_url: row.repo_url,
                revision: row.revision,
                path: row.path,
                public_key: row.public_key,
                fingerprint: row.fingerprint,
            })
            .collect();

        Ok(releases)
    }
}
