use std::sync::Mutex;

use tauri::State;

use crate::common::{crypto, state::AppState};

#[tauri::command]
pub fn unlock_vault(state: State<'_, Mutex<AppState>>, password: String) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    // 1. load salt from DB (stored plain, not secret)
    let salt = crypto::generate_salt(); // replace with DB load

    // 2. derive key from password + salt
    let key = crypto::derive_key(&password, &salt)?;

    // 3. store key in AppState (in-memory only, never persisted)
    state.set_key(key);

    println!(
        "Vault unlocked, derived key: {:#?}",
        key.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    );
    Ok(())
}

#[tauri::command]
pub fn add_entry(
    state: State<'_, Mutex<AppState>>,
    name: String,
    value: String,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let key = state.get_key()?; // fails if vault is locked

    // 4. encrypt value before writing to DB
    let encrypted = crypto::encrypt(&key, &value)?;
    println!("Encrypted value to store in DB: {}", encrypted);

    Ok(())
}

// #[tauri::command]
// pub fn get_entry_value(state: State<'_, Mutex<AppState>>, id: String) -> Result<String, String> {
//     let state = state.lock().unwrap();
//     let key = state.get_key()?;

//     // 5. read from DB and decrypt in Rust — never sends raw key to JS
//     let encrypted = db::get_entry(id)?;
//     let plaintext = crypto::decrypt(&key, &encrypted)?;

//     Ok(key) // only sent to JS when explicitly requested (copy action)
// }
