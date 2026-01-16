#[cfg(target_os = "macos")]
pub mod macos {
    use objc2_core_graphics::{CGEvent, CGEventFlags, CGEventTapLocation};

    const KEY_V: u16 = 9;

    pub fn simulate_paste() -> Result<(), String> {
        let source = CGEvent::new_keyboard_event(None, KEY_V, true)
            .ok_or("Failed to create key down event")?;

        CGEvent::set_flags(Some(&source), CGEventFlags::MaskCommand);
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&source));

        std::thread::sleep(std::time::Duration::from_millis(10));

        let source_up = CGEvent::new_keyboard_event(None, KEY_V, false)
            .ok_or("Failed to create key up event")?;

        CGEvent::set_flags(Some(&source_up), CGEventFlags::MaskCommand);
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&source_up));

        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
pub mod fallback {
    pub fn simulate_paste() -> Result<(), String> {
        Err("Text injection is only supported on macOS".to_string())
    }
}

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(target_os = "macos"))]
pub use fallback::*;
