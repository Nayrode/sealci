use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::log::{
        entities::log::{Log, LogError},
        ports::log_repository::LogRepository,
    },
    infrastructure::db::postgres::Postgres,
};

pub struct PostgresLogRepository {
    postgres: Arc<Postgres>,
}

impl PostgresLogRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

#[async_trait]
impl LogRepository for PostgresLogRepository {
    async fn create(&self, action_id: i64, data: String) -> Result<Log, LogError> {
        let row = sqlx::query_as!(
            Log,
            r#"INSERT INTO logs (action_id, data) VALUES ($1, $2) RETURNING *"#,
            action_id,
            data,
        )
        .fetch_one(&self.postgres.get_pool())
        .await
        .map_err(LogError::DatabaseError)?;

        Ok(Log {
            id: row.id,
            action_id: row.action_id,
            data: row.data,
        })
    }


    
    async fn find_by_action_id(&self, action_id: i64) -> Result<Vec<Log>, LogError> {
        let rows = sqlx::query_as!(Log, r#"SELECT * FROM logs WHERE action_id = $1"#, action_id)
            .fetch_all(&self.postgres.get_pool())
            .await
            .map_err(LogError::DatabaseError)?;

        let logs = rows
            .into_iter()
            .map(|row| Log {
                    id: row.id,
                    action_id: row.action_id,
                    data: row.data,
                },
            )
            .collect();

        Ok(logs)
    }
}
