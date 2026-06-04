use sqlx::{Executor, Pool, Sqlite};

use crate::db::orm::{Model, TableMetadata};

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
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

    println!("Rows affected: {}", result.rows_affected());
    Ok(())
}

pub async fn get_all_workspaces(pool: &Pool<Sqlite>) -> Result<Vec<Workspace>, sqlx::Error> {
    let workspac_model = Model::<Workspace>::new(pool);
    let found = workspac_model.find_all(vec![]).await.unwrap();
    return Ok(found);
}
