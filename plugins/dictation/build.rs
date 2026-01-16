const COMMANDS: &[&str] = &[
    "start_dictation",
    "stop_dictation",
    "cancel_dictation",
    "get_dictation_state",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
