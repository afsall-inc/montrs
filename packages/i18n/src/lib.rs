//! MontRS internationalization — runtime translation loading, locale switching,
//! pluralization, scoping, and interpolation.
//!
//! # Features
//! - Load translations from JSON, TOML files at runtime
//! - Locale detection from Accept-Language, URL path
//! - Variable interpolation (`{name}` in templates)
//! - Pluralization (zero, one, few, many, other forms)
//! - Scoping (prefix-based key namespaces)
//! - Language negotiation for SSR
//!
//! # Example
//! ```rust,ignore
//! use montrs_i18n::{I18nContext, Locale, Locales};
//!
//! let ctx = I18nContext::from_dir("locales", "en", &["en", "fr"], "json")?;
//! let greeting = ctx.t("greeting", &[("name", "World")])?;
//! ```
//!
//! # Translation file format (JSON)
//! ```json
//! {
//!   "greeting": "Hello, {name}!",
//!   "items.one": "You have 1 item",
//!   "items.other": "You have {count} items"
//! }
//! ```

mod context;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A locale identifier.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
)]
pub struct Locale {
    pub code: String,
    pub display_name: String,
}

impl Locale {
    pub fn new(code: &str, display_name: &str) -> Self {
        Self {
            code: code.to_string(),
            display_name: display_name.to_string(),
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.display_name, self.code)
    }
}

/// A map of translation keys to values for a single locale.
pub type TranslationMap = IndexMap<String, String>;

/// All loaded translations keyed by locale code.
pub type LocaleTranslations = HashMap<String, TranslationMap>;

/// Errors from i18n operations.
#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    #[error("Locale '{0}' not found")]
    LocaleNotFound(String),
    #[error("Translation key '{0}' not found")]
    KeyNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Missing interpolation variable: {0}")]
    MissingVar(String),
}

pub use context::{
    I18nContext, Locales, ScopedContext, interpolate,
    resolve_locale_from_request,
};
