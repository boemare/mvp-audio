use tauri::Manager;

use crate::state::{HotkeyConfig, HotkeysState};

#[tauri::command]
#[specta::specta]
pub async fn register_hotkey<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    config: HotkeyConfig,
) -> Result<(), String> {
    let state = app.state::<HotkeysState>();
    state.register_hotkey(config).await;
    tracing::info!("hotkey_registered");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn unregister_hotkey<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    id: String,
) -> Result<bool, String> {
    let state = app.state::<HotkeysState>();
    let removed = state.unregister_hotkey(&id).await;
    tracing::info!(removed, "hotkey_unregistered");
    Ok(removed)
}

#[tauri::command]
#[specta::specta]
pub async fn get_registered_hotkeys<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<Vec<HotkeyConfig>, String> {
    let state = app.state::<HotkeysState>();
    Ok(state.get_registered_hotkeys().await)
}

#[tauri::command]
#[specta::specta]
pub async fn start_listening<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let state = app.state::<HotkeysState>();
    state.set_listening(true).await;
    tracing::info!("hotkey_listening_started");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn stop_listening<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let state = app.state::<HotkeysState>();
    state.set_listening(false).await;
    tracing::info!("hotkey_listening_stopped");
    Ok(())
}
