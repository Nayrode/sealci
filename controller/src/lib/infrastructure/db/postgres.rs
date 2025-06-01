use sqlx::PgPool;

pub struct Postgres {
    pub pool: PgPool,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url).await.unwrap();
        Self { pool: pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
