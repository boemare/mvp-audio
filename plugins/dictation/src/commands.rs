use tauri::Manager;

use crate::events::{DictationPhase, DictationResult};
use crate::state::DictationState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct StartDictationParams {
    pub language: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn start_dictation<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    params: StartDictationParams,
) -> Result<(), String> {
    let state = app.state::<DictationState>();

    state.start_recording(16000).await.map_err(|e| e.to_string())?;

    tracing::info!(?params, "dictation_started");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn stop_dictation<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<DictationResult, String> {
    let state = app.state::<DictationState>();

    let (samples, sample_rate, duration_ms) = state.stop_recording().await.map_err(|e| e.to_string())?;

    let raw_text = format!(
        "[Transcription of {} samples at {}Hz]",
        samples.len(),
        sample_rate
    );
    let processed_text = raw_text.clone();

    state.complete().await;

    tracing::info!(duration_ms, "dictation_stopped");

    Ok(DictationResult {
        raw_text,
        processed_text,
        duration_ms,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn cancel_dictation<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let state = app.state::<DictationState>();
    state.cancel().await;

    tracing::info!("dictation_cancelled");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_dictation_state<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<DictationPhase, String> {
    let state = app.state::<DictationState>();
    Ok(state.get_phase().await)
}
