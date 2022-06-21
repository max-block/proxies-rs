use sqlx::{postgres::PgPoolOptions, PgPool, Result};

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
        Ok(Db { pool })
    }
}
