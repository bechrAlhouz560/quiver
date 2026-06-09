use crate::{
    common::state::AppState,
    db::{
        db_state::DbState,
        models::workspace::{Workspace, WorkspaceInput},
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
    workspace: WorkspaceInput,
) -> Result<Workspace, String> {
    let pool = {
        let state = state.lock().unwrap();
        state.get_pool().map_err(|err| err.to_string())?.clone()
    }; // lock dropped here
    let workspace_model = Model::<Workspace>::new(&pool);
    let workspace_id = workspace_model
        .create(&[
            (
                "name",
                Box::new(Field {
                    value: workspace.name.clone(),
                }) as Box<dyn Boxable + Send + Sync>,
            ),
            (
                "description",
                Box::new(Field {
                    value: workspace.description.clone().unwrap_or_default(),
                }) as Box<dyn Boxable + Send + Sync>,
            ),
            (
                "created_at",
                Box::new(Field {
                    value: workspace.created_at.clone(),
                }) as Box<dyn Boxable + Send + Sync>,
            ),
        ])
        .await
        .map_err(|err| err.to_string())?;
    Ok(Workspace {
        id: workspace_id,
        name: workspace.name,
        description: workspace.description,
        created_at: workspace.created_at,
    })
}

#[tauri::command]
pub async fn get_workspaces(state: State<'_, Mutex<DbState>>) -> Result<Vec<Workspace>, String> {
    let pool = {
        let state = state.lock().unwrap();
        state.get_pool().map_err(|err| err.to_string())?.clone()
    }; // lock dropped here

    let workspace_model = Model::<Workspace>::new(&pool);

    let workspaces = workspace_model
        .find_all(vec![], None)
        .await
        .map_err(|err| err.to_string())
        .unwrap();

    Ok(workspaces)
}
