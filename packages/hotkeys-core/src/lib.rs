//! Platform-independent keyboard shortcut parsing and matching.

use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Control,
    Alt,
    Shift,
    Meta,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hotkey {
    pub modifiers: Vec<Modifier>,
    pub key: String,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HotkeyError {
    #[error("hotkey is empty")]
    Empty,
    #[error("hotkey must contain a non-modifier key")]
    MissingKey,
    #[error("unknown hotkey part: {0}")]
    UnknownPart(String),
}

impl Hotkey {
    pub fn parse(input: &str) -> Result<Self, HotkeyError> {
        if input.trim().is_empty() {
            return Err(HotkeyError::Empty);
        }
        let mut modifiers = Vec::new();
        let mut key = None;
        for part in input.split('+').map(str::trim).filter(|part| !part.is_empty()) {
            let normalized = part.to_ascii_lowercase();
            let modifier = match normalized.as_str() {
                "ctrl" | "control" => Some(Modifier::Control),
                "alt" | "option" => Some(Modifier::Alt),
                "shift" => Some(Modifier::Shift),
                "meta" | "cmd" | "command" | "super" | "win" | "mod" => {
                    Some(Modifier::Meta)
                }
                _ => None,
            };
            if let Some(modifier) = modifier {
                if !modifiers.contains(&modifier) {
                    modifiers.push(modifier);
                }
            } else if key.replace(part.to_string()).is_some() {
                return Err(HotkeyError::UnknownPart(part.to_string()));
            }
        }
        let key = key.ok_or(HotkeyError::MissingKey)?;
        Ok(Self { modifiers, key })
    }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.key.eq_ignore_ascii_case(&event.key)
            && self.modifiers.iter().all(|modifier| event.modifiers.contains(modifier))
            && event.modifiers.len() == self.modifiers.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyEvent {
    pub key: String,
    pub modifiers: Vec<Modifier>,
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, modifier) in self.modifiers.iter().enumerate() {
            if index > 0 {
                write!(f, "+")?;
            }
            write!(f, "{modifier:?}")?;
        }
        if !self.modifiers.is_empty() {
            write!(f, "+")?;
        }
        write!(f, "{}", self.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mod_alias_and_deduplicates_modifiers() {
        let hotkey = Hotkey::parse("Mod+Shift+P+Shift").unwrap();
        assert_eq!(hotkey.key, "P");
        assert_eq!(hotkey.modifiers, vec![Modifier::Meta, Modifier::Shift]);
    }

    #[test]
    fn matches_exact_modifier_set() {
        let hotkey = Hotkey::parse("Ctrl+K").unwrap();
        assert!(hotkey.matches(&KeyEvent {
            key: "k".into(),
            modifiers: vec![Modifier::Control],
        }));
        assert!(!hotkey.matches(&KeyEvent {
            key: "k".into(),
            modifiers: vec![Modifier::Control, Modifier::Shift],
        }));
    }
}
