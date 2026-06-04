use std::sync::Mutex;
mod commands;
mod common;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Mutex::new(common::state::AppState::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::tester::tester,
            // vault commands
            commands::vault::unlock_vault,
            // workspace commands
            commands::workspace::get_active_workspace,
            commands::workspace::set_active_workspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
