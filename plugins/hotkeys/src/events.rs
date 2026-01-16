use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct HotkeyPressedEvent {
    pub hotkey_id: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct HotkeyReleasedEvent {
    pub hotkey_id: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct HotkeyActionEvent {
    pub hotkey_id: String,
    pub action: HotkeyAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum HotkeyAction {
    Activate,
    Deactivate,
    Lock,
    Unlock,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct KeysHeldEvent {
    pub keys: Vec<String>,
}
