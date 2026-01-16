use std::sync::Arc;
use tauri::Manager;

mod commands;
mod events;
mod listener;
mod state;

pub use events::*;
pub use state::{HotkeyConfig, HotkeysState};

const PLUGIN_NAME: &str = "hotkeys";

fn make_specta_builder<R: tauri::Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::<R>::new()
        .plugin_name(PLUGIN_NAME)
        .commands(tauri_specta::collect_commands![
            commands::register_hotkey::<tauri::Wry>,
            commands::unregister_hotkey::<tauri::Wry>,
            commands::get_registered_hotkeys::<tauri::Wry>,
            commands::start_listening::<tauri::Wry>,
            commands::stop_listening::<tauri::Wry>,
        ])
        .events(tauri_specta::collect_events![
            HotkeyPressedEvent,
            HotkeyReleasedEvent,
            HotkeyActionEvent,
            KeysHeldEvent,
        ])
        .error_handling(tauri_specta::ErrorHandlingMode::Result)
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    let specta_builder = make_specta_builder();

    tauri::plugin::Builder::new(PLUGIN_NAME)
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app, _api| {
            specta_builder.mount_events(app);

            let state = Arc::new(HotkeysState::new());
            app.manage(state.clone());

            let app_handle = app.clone();
            listener::start_global_listener(app_handle, state);

            tracing::info!("hotkeys_plugin_initialized");
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn export_types() {
        const OUTPUT_FILE: &str = "./js/bindings.gen.ts";

        make_specta_builder::<tauri::Wry>()
            .export(
                specta_typescript::Typescript::default()
                    .formatter(specta_typescript::formatter::prettier)
                    .bigint(specta_typescript::BigIntExportBehavior::Number),
                OUTPUT_FILE,
            )
            .unwrap();

        let content = std::fs::read_to_string(OUTPUT_FILE).unwrap();
        std::fs::write(OUTPUT_FILE, format!("// @ts-nocheck\n{content}")).unwrap();
    }
}
