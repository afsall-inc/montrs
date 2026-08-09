//! Stub helpers for internal macro implementations.

use std::fmt;

/// A literal wrapper for static strings from translations.
#[derive(Debug, Clone, Copy)]
pub struct LitWrapper(pub &'static str);

impl LitWrapper {
    pub const fn new(s: &'static str) -> Self {
        LitWrapper(s)
    }
    pub fn inner(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for LitWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Interpolation argument struct.
pub struct InterpolArgs<F> {
    pub key: &'static str,
    pub value: F,
}

/// Interpolated display builder.
pub struct InterpolatedDisplay {
    pub template: &'static str,
}

impl fmt::Display for InterpolatedDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.template)
    }
}

/// Get component from key path.
pub fn get_key_component(key: &str) -> &str {
    key
}
