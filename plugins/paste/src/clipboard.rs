#[cfg(target_os = "macos")]
pub mod macos {
    use objc2_app_kit::NSPasteboard;
    use objc2_foundation::NSString;

    pub fn get_clipboard_text() -> Option<String> {
        let pasteboard = NSPasteboard::generalPasteboard();
        let types = pasteboard.types()?;

        let string_type = NSString::from_str("public.utf8-plain-text");
        if !types.containsObject(&string_type) {
            return None;
        }

        let content = pasteboard.stringForType(&string_type)?;
        Some(content.to_string())
    }

    pub fn set_clipboard_text(text: &str) -> bool {
        let pasteboard = NSPasteboard::generalPasteboard();
        pasteboard.clearContents();

        let ns_string = NSString::from_str(text);
        let string_type = NSString::from_str("public.utf8-plain-text");

        pasteboard.setString_forType(&ns_string, &string_type)
    }
}

#[cfg(not(target_os = "macos"))]
pub mod fallback {
    pub fn get_clipboard_text() -> Option<String> {
        None
    }

    pub fn set_clipboard_text(_text: &str) -> bool {
        false
    }
}

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(target_os = "macos"))]
pub use fallback::*;
