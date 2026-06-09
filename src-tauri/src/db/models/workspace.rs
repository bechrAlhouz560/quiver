use sqlx::{Executor, Pool, Sqlite};

use crate::db::orm::TableMetadata;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceInput {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}
impl TableMetadata for Workspace {
    const TABLE_NAME: &'static str = "workspaces";
}
pub async fn init_workspace_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
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

    // here will be modifications or additions queries
    let _ = pool
        .execute(
            "
             ALTER TABLE workspaces ADD COLUMN created_at TIMESTAMP
    ",
        )
        .await
        .is_ok();

    println!("Rows affected: {}", result.rows_affected());
    Ok(())
}
