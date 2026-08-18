//! Browser adapter for MontRS hotkeys.

use montrs_hotkeys_core::{Hotkey, HotkeyError};
#[cfg(target_arch = "wasm32")]
use montrs_hotkeys_core::{KeyEvent, Modifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserHotkey {
    pub hotkey: Hotkey,
    pub prevent_default: bool,
}

impl BrowserHotkey {
    pub fn parse(input: &str) -> Result<Self, HotkeyError> {
        Ok(Self {
            hotkey: Hotkey::parse(input)?,
            prevent_default: false,
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn matches(&self, event: &web_sys::KeyboardEvent) -> bool {
        let mut modifiers = Vec::new();
        if event.ctrl_key() {
            modifiers.push(Modifier::Control);
        }
        if event.alt_key() {
            modifiers.push(Modifier::Alt);
        }
        if event.shift_key() {
            modifiers.push(Modifier::Shift);
        }
        if event.meta_key() {
            modifiers.push(Modifier::Meta);
        }
        self.hotkey.matches(&KeyEvent {
            key: event.key(),
            modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_browser_hotkey() {
        assert!(BrowserHotkey::parse("Mod+K").is_ok());
    }
}
