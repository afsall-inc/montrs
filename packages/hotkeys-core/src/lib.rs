//! Platform-independent keyboard shortcut parsing and matching.
//! API modeled on leptos-hotkeys and tanstack-hotkeys: parse, normalize, match.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

/// All modifier flags as booleans (matches leptos-hotkeys `KeyboardModifiers`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyboardModifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub shift: bool,
}

impl KeyboardModifiers {
    pub fn none() -> Self { Self::default() }
    pub fn is_empty(self) -> bool { !self.alt && !self.ctrl && !self.meta && !self.shift }
}

impl fmt::Display for KeyboardModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.ctrl { parts.push("Ctrl"); }
        if self.alt { parts.push("Alt"); }
        if self.shift { parts.push("Shift"); }
        if self.meta { parts.push("Meta"); }
        if parts.is_empty() { return Ok(()); }
        write!(f, "{}", parts.join("+"))
    }
}

/// Type alias matching the reference.
pub type Keys = Vec<String>;

/// Parse error for hotkey strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseHotkeyError;

impl fmt::Display for ParseHotkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "invalid hotkey") }
}
impl std::error::Error for ParseHotkeyError {}

/// A single, parsed key combination such as `Ctrl+Shift+K`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hotkey {
    pub(crate) modifiers: KeyboardModifiers,
    pub(crate) keys: Keys,
}

impl Hotkey {
    /// Parse a key combination string like `"ctrl+shift+k"`.
    pub fn new(key_combination: &str) -> Self {
        Self::parse_string(key_combination)
    }

    fn parse_string(input: &str) -> Self {
        let mut modifiers = KeyboardModifiers::default();
        let mut keys = Keys::new();
        for part in input.split('+').map(str::trim) {
            match part.to_ascii_lowercase().as_str() {
                "control" | "ctrl" => modifiers.ctrl = true,
                "alt" | "option" => modifiers.alt = true,
                "meta" | "cmd" | "command" | "super" | "win" | "mod" => modifiers.meta = true,
                "shift" => modifiers.shift = true,
                other => keys.push(normalize_key(other)),
            }
        }
        if keys.is_empty() {
            keys.push("".to_string());
        }
        Self { modifiers, keys }
    }

    /// All modifiers in a canonical order.
    pub fn modifiers(&self) -> KeyboardModifiers { self.modifiers }

    /// The non-modifier keys.
    pub fn keys(&self) -> &[String] { &self.keys }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        if self.modifiers != event.modifiers { return false; }
        if self.keys.is_empty() { return event.key.is_empty(); }
        self.keys.iter().any(|key| key.eq_ignore_ascii_case(&event.key))
    }
}

fn normalize_key(key: &str) -> String {
    match key {
        " " => "spacebar".to_string(),
        "space" => "spacebar".to_string(),
        other => other.to_ascii_lowercase(),
    }
}

impl FromStr for Hotkey {
    type Err = ParseHotkeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() { return Err(ParseHotkeyError); }
        Ok(Self::parse_string(s))
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = self.keys.clone();
        for key in self.keys() {
            parts.push(key.clone());
        }
        let mods = self.modifiers.to_string();
        if mods.is_empty() {
            write!(f, "{}", self.keys.join("+"))
        } else {
            write!(f, "{}+{}", self.keys.join("+"), mods)
        }
    }
}

/// Key press state used for chord matching.
#[derive(Debug, Clone, Default)]
pub struct KeyPresses {
    pub key_map: BTreeMap<String, String>,
    pub last_key: Option<String>,
}

impl KeyPresses {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, key: String) {
        self.key_map.insert(key.clone(), key.clone());
        self.last_key = Some(key);
    }
    pub fn release(&mut self, key: &str) { self.key_map.remove(key); }
    pub fn clear(&mut self) { self.key_map.clear(); self.last_key = None; }
    pub fn is_empty(&self) -> bool { self.key_map.is_empty() }
}

/// A normalized keyboard event for matching.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyEvent {
    pub key: String,
    pub modifiers: KeyboardModifiers,
}

impl KeyEvent {
    pub fn new(key: impl Into<String>, modifiers: KeyboardModifiers) -> Self {
        Self { key: normalize_key(&key.into()), modifiers }
    }
}

/// True when the last pressed key in the press set is the hotkey's final key.
pub fn is_last_key_match(parsed: &[Hotkey], pressed: &KeyPresses) -> bool {
    let Some(last) = &pressed.last_key else { return false };
    parsed.iter().any(|hotkey| hotkey.keys.iter().any(|k| k == last))
}

/// True when the full modifier set and key combination match the pressed set.
pub fn is_hotkey_match(hotkey: &Hotkey, pressed: &KeyPresses) -> bool {
    let Some(last) = &pressed.last_key else { return false };
    hotkey.keys.iter().any(|k| k == last)
        && pressed.key_map.len() == hotkey.keys.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mod_aliases() {
        let hk = Hotkey::new("Mod+Shift+P");
        assert!(hk.modifiers.meta);
        assert!(hk.modifiers.shift);
        assert_eq!(hk.keys(), &["p"]);
        assert!(hk.matches(&KeyEvent::new("P", KeyboardModifiers { meta: true, shift: true, ..Default::default() })));
    }

    #[test]
    fn from_str_and_display() {
        let hk: Hotkey = "ctrl+k".parse().unwrap();
        assert!(hk.modifiers.ctrl);
        assert_eq!(hk.keys(), &["k"]);
    }

    #[test]
    fn matches_exact_modifiers() {
        let hk = Hotkey::new("Ctrl+K");
        assert!(hk.matches(&KeyEvent::new("k", KeyboardModifiers { ctrl: true, ..Default::default() })));
        assert!(!hk.matches(&KeyEvent::new("k", KeyboardModifiers { ctrl: true, shift: true, ..Default::default() })));
    }
}
