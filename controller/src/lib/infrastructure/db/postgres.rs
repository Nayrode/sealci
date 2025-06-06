use sqlx::PgPool;

use crate::application::AppError;

pub struct Postgres {
    pub pool: PgPool,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(AppError::DatabaseConnectionError)?;
        Ok(Self { pool: pool }) 
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
