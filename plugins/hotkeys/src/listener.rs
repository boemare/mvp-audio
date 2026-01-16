use rdev::{listen, Event, EventType, Key};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::events::{HotkeyActionEvent, KeysHeldEvent};
use crate::state::HotkeysState;

pub enum ListenerCommand {
    Stop,
}

fn key_to_string(key: Key) -> String {
    match key {
        Key::Alt => "Alt".to_string(),
        Key::AltGr => "AltGr".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::CapsLock => "CapsLock".to_string(),
        Key::ControlLeft => "Control".to_string(),
        Key::ControlRight => "Control".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::DownArrow => "ArrowDown".to_string(),
        Key::End => "End".to_string(),
        Key::Escape => "Escape".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        Key::Home => "Home".to_string(),
        Key::LeftArrow => "ArrowLeft".to_string(),
        Key::MetaLeft => "Meta".to_string(),
        Key::MetaRight => "Meta".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::Return => "Enter".to_string(),
        Key::RightArrow => "ArrowRight".to_string(),
        Key::ShiftLeft => "Shift".to_string(),
        Key::ShiftRight => "Shift".to_string(),
        Key::Space => "Space".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::UpArrow => "ArrowUp".to_string(),
        Key::PrintScreen => "PrintScreen".to_string(),
        Key::ScrollLock => "ScrollLock".to_string(),
        Key::Pause => "Pause".to_string(),
        Key::NumLock => "NumLock".to_string(),
        Key::BackQuote => "`".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        Key::Num0 => "0".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::KeyQ => "Q".to_string(),
        Key::KeyW => "W".to_string(),
        Key::KeyE => "E".to_string(),
        Key::KeyR => "R".to_string(),
        Key::KeyT => "T".to_string(),
        Key::KeyY => "Y".to_string(),
        Key::KeyU => "U".to_string(),
        Key::KeyI => "I".to_string(),
        Key::KeyO => "O".to_string(),
        Key::KeyP => "P".to_string(),
        Key::LeftBracket => "[".to_string(),
        Key::RightBracket => "]".to_string(),
        Key::KeyA => "A".to_string(),
        Key::KeyS => "S".to_string(),
        Key::KeyD => "D".to_string(),
        Key::KeyF => "F".to_string(),
        Key::KeyG => "G".to_string(),
        Key::KeyH => "H".to_string(),
        Key::KeyJ => "J".to_string(),
        Key::KeyK => "K".to_string(),
        Key::KeyL => "L".to_string(),
        Key::SemiColon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::BackSlash => "\\".to_string(),
        Key::IntlBackslash => "\\".to_string(),
        Key::KeyZ => "Z".to_string(),
        Key::KeyX => "X".to_string(),
        Key::KeyC => "C".to_string(),
        Key::KeyV => "V".to_string(),
        Key::KeyB => "B".to_string(),
        Key::KeyN => "N".to_string(),
        Key::KeyM => "M".to_string(),
        Key::Comma => ",".to_string(),
        Key::Dot => ".".to_string(),
        Key::Slash => "/".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::KpReturn => "NumpadEnter".to_string(),
        Key::KpMinus => "NumpadSubtract".to_string(),
        Key::KpPlus => "NumpadAdd".to_string(),
        Key::KpMultiply => "NumpadMultiply".to_string(),
        Key::KpDivide => "NumpadDivide".to_string(),
        Key::Kp0 => "Numpad0".to_string(),
        Key::Kp1 => "Numpad1".to_string(),
        Key::Kp2 => "Numpad2".to_string(),
        Key::Kp3 => "Numpad3".to_string(),
        Key::Kp4 => "Numpad4".to_string(),
        Key::Kp5 => "Numpad5".to_string(),
        Key::Kp6 => "Numpad6".to_string(),
        Key::Kp7 => "Numpad7".to_string(),
        Key::Kp8 => "Numpad8".to_string(),
        Key::Kp9 => "Numpad9".to_string(),
        Key::KpDelete => "NumpadDecimal".to_string(),
        Key::Function => "Fn".to_string(),
        Key::Unknown(code) => format!("Unknown({})", code),
    }
}

pub fn start_global_listener<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    state: Arc<HotkeysState>,
) -> mpsc::Sender<ListenerCommand> {
    let (tx, mut rx) = mpsc::channel::<ListenerCommand>(10);

    std::thread::spawn(move || {
        let callback = move |event: Event| {
            let state = state.clone();
            let app = app_handle.clone();

            match event.event_type {
                EventType::KeyPress(key) => {
                    let key_str = key_to_string(key);
                    let state_clone = state.clone();
                    let app_clone = app.clone();

                    tokio::spawn(async move {
                        if !state_clone.is_listening().await {
                            return;
                        }

                        let actions = state_clone.on_key_event(key_str, true).await;

                        for (hotkey_id, action) in actions {
                            let _ = tauri_specta::Event::emit(
                                &HotkeyActionEvent { hotkey_id, action },
                                &app_clone,
                            );
                        }

                        let pressed = state_clone.get_pressed_keys().await;
                        let _ = tauri_specta::Event::emit(
                            &KeysHeldEvent { keys: pressed },
                            &app_clone,
                        );
                    });
                }
                EventType::KeyRelease(key) => {
                    let key_str = key_to_string(key);
                    let state_clone = state.clone();
                    let app_clone = app.clone();

                    tokio::spawn(async move {
                        if !state_clone.is_listening().await {
                            return;
                        }

                        let actions = state_clone.on_key_event(key_str, false).await;

                        for (hotkey_id, action) in actions {
                            let _ = tauri_specta::Event::emit(
                                &HotkeyActionEvent { hotkey_id, action },
                                &app_clone,
                            );
                        }

                        let pressed = state_clone.get_pressed_keys().await;
                        let _ = tauri_specta::Event::emit(
                            &KeysHeldEvent { keys: pressed },
                            &app_clone,
                        );
                    });
                }
                _ => {}
            }
        };

        if let Err(error) = listen(callback) {
            tracing::error!(?error, "global_key_listener_error");
        }
    });

    std::thread::spawn(move || {
        while let Some(cmd) = rx.blocking_recv() {
            match cmd {
                ListenerCommand::Stop => {
                    tracing::info!("global_key_listener_stopped");
                    break;
                }
            }
        }
    });

    tx
}
