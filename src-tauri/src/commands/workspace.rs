use crate::common::state::AppState;
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
