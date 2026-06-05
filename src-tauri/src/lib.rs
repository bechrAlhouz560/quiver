use std::sync::Mutex;

use tauri::Manager;

use crate::db::db_state::DbState;
mod commands;
mod common;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Mutex::new(common::state::AppState::default());
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .setup(|app| {
            // database connectivoty
            let app_handle = app.handle().clone();

            // get the app data directory for the db file
            let data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&data_dir)?;

            let db_url = format!("sqlite://{}", data_dir.join("quiver.db").to_string_lossy());

            // connect synchronously using block_in_place inside setup
            let db_state = tauri::async_runtime::block_on(async {
                let mut state = DbState::new();
                state
                    .connect(&db_url)
                    .await
                    .expect("Failed to connect to DB");
                state
            });

            app.manage(Mutex::new(db_state));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tester::tester,
            // vault commands
            commands::vault::unlock_vault,
            // workspace commands
            commands::workspace::get_active_workspace,
            commands::workspace::set_active_workspace,
            commands::workspace::create_workspace,
            // database commands
            commands::database::init_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
