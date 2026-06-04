use sqlx::{Executor, Pool, Sqlite};

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Vault {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

pub async fn init_vault_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Fixed the SQL string to use valid SQLite syntax
    let result = pool
        .execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            name TEXT NOT NULL, 
            description TEXT
        );",
        )
        .await?;

    println!("Rows affected: {}", result.rows_affected());
    Ok(())
}
