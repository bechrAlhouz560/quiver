use std::sync::Mutex;
use tauri::State;

use crate::db::{db_state::DbState, models::workspace::init_workspace_table};

#[tauri::command]
pub async fn init_database(state: State<'_, Mutex<DbState>>) -> Result<String, String> {
    let pool = {
        let lock = state.lock().map_err(|e| e.to_string())?;
        lock.get_pool()?.clone()
    };
    init_workspace_table(&pool)
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(String::from("Suucess"))
}
