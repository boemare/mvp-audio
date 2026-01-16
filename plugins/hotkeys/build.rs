const COMMANDS: &[&str] = &[
    "register_hotkey",
    "unregister_hotkey",
    "get_registered_hotkeys",
    "start_listening",
    "stop_listening",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
