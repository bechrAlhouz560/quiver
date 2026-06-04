#[derive(Default)]
pub struct AppState {
    // unlock key
    key: Option<[u8; 32]>,
    // current active key
    workspace_id: String,
}

impl AppState {
    // key management
    pub fn get_key(&self) -> Result<&[u8; 32], String> {
        self.key
            .as_ref()
            .ok_or_else(|| "Vault is locked".to_string())
    }

    pub fn set_key(&mut self, key: [u8; 32]) {
        self.key = Some(key);
    }

    // workspace management
    pub fn get_workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub fn set_workspace_id(&mut self, workspace_id: String) {
        self.workspace_id = workspace_id;
    }
}
