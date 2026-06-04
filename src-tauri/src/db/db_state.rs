use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::str::FromStr;

pub struct DbState {
    pool: Option<Pool<Sqlite>>,
}

impl DbState {
    pub fn new() -> Self {
        Self { pool: None }
    }

    pub async fn connect(&mut self, db_url: &str) -> Result<(), sqlx::Error> {
        let options = SqliteConnectOptions::from_str(db_url)?.create_if_missing(true);
        self.pool = Some(SqlitePool::connect_with(options).await?);
        Ok(())
    }

    pub fn get_pool(&self) -> Result<&Pool<Sqlite>, String> {
        self.pool
            .as_ref()
            .ok_or_else(|| "Database not connected".to_string())
    }
}
