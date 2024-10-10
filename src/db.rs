use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub struct DbService {
    pool: SqlitePool,
}

impl DbService {
    pub async fn connect(filename: &Path) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
}
