const COMMANDS: &[&str] = &["inject_text", "get_clipboard_text", "set_clipboard_text"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
