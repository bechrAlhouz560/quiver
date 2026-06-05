use crate::{
    common::state::AppState,
    db::{
        db_state::DbState,
        models::workspace::Workspace,
        orm::{Boxable, Field, Model},
    },
};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_active_workspace(state: State<'_, Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    state.get_workspace_id().to_string() // owned, safe to return
}

#[tauri::command]
pub fn set_active_workspace(state: State<'_, Mutex<AppState>>, workspace_id: String) {
    let mut state = state.lock().unwrap();
    state.set_workspace_id(workspace_id);
}

#[tauri::command]
pub async fn create_workspace(
    state: State<'_, Mutex<DbState>>,
    workspace: Workspace,
) -> Result<i64, String> {
    let pool = {
        let state = state.lock().unwrap();
        state.get_pool().map_err(|err| err.to_string())?.clone()
    }; // lock dropped here
    let workspace_model = Model::<Workspace>::new(&pool);
    let result = workspace_model
        .create(&[
            (
                "name",
                Box::new(Field {
                    value: workspace.name,
                }) as Box<dyn Boxable + Send + Sync>,
            ),
            (
                "description",
                Box::new(Field {
                    value: workspace.description.unwrap_or_default(),
                }) as Box<dyn Boxable + Send + Sync>,
            ),
        ])
        .await
        .map_err(|err| err.to_string())?;

    Ok(result)
}
