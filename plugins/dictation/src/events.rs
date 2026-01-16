use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct DictationAmplitudeEvent {
    pub levels: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct DictationStateEvent {
    pub state: DictationPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct DictationCompleteEvent {
    pub result: DictationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct DictationErrorEvent {
    pub error: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum DictationPhase {
    Idle,
    Recording,
    Locked,
    Processing,
}

impl Default for DictationPhase {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct DictationResult {
    pub raw_text: String,
    pub processed_text: String,
    pub duration_ms: u64,
}
