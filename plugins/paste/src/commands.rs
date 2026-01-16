use crate::{clipboard, keyboard};

#[tauri::command]
#[specta::specta]
pub async fn inject_text(text: String) -> Result<(), String> {
    let old_clipboard = clipboard::get_clipboard_text();

    if !clipboard::set_clipboard_text(&text) {
        return Err("Failed to set clipboard text".to_string());
    }

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    keyboard::simulate_paste()?;

    if let Some(old_text) = old_clipboard {
        let old_text_clone = old_text.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(800)).await;
            clipboard::set_clipboard_text(&old_text_clone);
        });
    }

    tracing::info!(len = text.len(), "text_injected");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_clipboard_text() -> Result<Option<String>, String> {
    Ok(clipboard::get_clipboard_text())
}

#[tauri::command]
#[specta::specta]
pub async fn set_clipboard_text(text: String) -> Result<bool, String> {
    Ok(clipboard::set_clipboard_text(&text))
}
