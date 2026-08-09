//! I18nContext — manages locale state and provides translations.

use crate::{I18nError, Locale, LocaleTranslations, TranslationMap};
use std::{collections::HashMap, path::Path};

/// The internationalization context — holds translations and active locale.
#[derive(Clone)]
pub struct I18nContext {
    /// The currently active locale code.
    pub locale_code: String,
    /// All loaded translations, keyed by locale code.
    pub translations: LocaleTranslations,
    /// All available locales.
    pub locales: Vec<Locale>,
    /// The default locale code.
    pub default_locale: String,
}

impl I18nContext {
    /// Create a new context from pre-loaded translations.
    pub fn new(
        translations: LocaleTranslations,
        default_locale: &str,
        locales: Vec<Locale>,
    ) -> Self {
        Self {
            locale_code: default_locale.to_string(),
            translations,
            locales,
            default_locale: default_locale.to_string(),
        }
    }

    /// Create from a directory of translation files (e.g., `locales/en.json`, `locales/fr.json`).
    pub fn from_dir(
        dir: &Path,
        default_locale: &str,
        locale_codes: &[&str],
        format: &str,
    ) -> Result<Self, I18nError> {
        let mut translations: LocaleTranslations = HashMap::new();
        let mut locales = Vec::new();

        for code in locale_codes {
            let file = dir.join(format!("{code}.{format}"));
            if !file.exists() {
                continue;
            }
            let content = std::fs::read_to_string(&file)?;
            let map: TranslationMap = match format {
                "json" => serde_json::from_str(&content)
                    .map_err(|e| I18nError::Parse(e.to_string()))?,
                "toml" => toml::from_str(&content)
                    .map_err(|e| I18nError::Parse(e.to_string()))?,
                _ => {
                    return Err(I18nError::Parse(format!(
                        "unsupported format: {format}"
                    )));
                }
            };
            translations.insert(code.to_string(), map);
            locales.push(Locale::new(code, code));
        }

        Ok(Self::new(translations, default_locale, locales))
    }

    /// Translate a key with optional interpolation variables.
    pub fn t(
        &self,
        key: &str,
        vars: &[(&str, &str)],
    ) -> Result<String, I18nError> {
        let map = self
            .translations
            .get(&self.locale_code)
            .or_else(|| self.translations.get(&self.default_locale))
            .ok_or_else(|| {
                I18nError::LocaleNotFound(self.locale_code.clone())
            })?;

        let template = map
            .get(key)
            .ok_or_else(|| I18nError::KeyNotFound(key.to_string()))?;

        Ok(interpolate(template, vars))
    }

    /// Translate with pluralization.
    pub fn t_plural(
        &self,
        key: &str,
        count: i64,
        vars: &[(&str, &str)],
    ) -> Result<String, I18nError> {
        let form = match count {
            0 => "zero",
            1 => "one",
            2 => "two",
            3..=10 => "few",
            _ => "other",
        };
        let mut all_vars: Vec<(&str, String)> =
            vars.iter().map(|(k, v)| (*k, v.to_string())).collect();
        all_vars.push(("count", count.to_string()));
        let refs: Vec<(&str, &str)> =
            all_vars.iter().map(|(k, v)| (&k[..], &v[..])).collect();

        // Try specific form, then "other", then fall back to the map's default.
        for suffix in [form, "other"] {
            let plural_key = format!("{key}.{suffix}");
            if let Ok(result) = self.t(&plural_key, &refs) {
                return Ok(result);
            }
        }
        Err(I18nError::KeyNotFound(format!("{key}.{form}")))
    }

    /// Set the active locale.
    pub fn set_locale(&mut self, code: &str) -> Result<(), I18nError> {
        if !self.locales.iter().any(|l| l.code == code) {
            return Err(I18nError::LocaleNotFound(code.to_string()));
        }
        self.locale_code = code.to_string();
        Ok(())
    }

    /// Get all translation keys for the current locale.
    pub fn keys(&self) -> Vec<String> {
        self.translations
            .get(&self.locale_code)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Get the current locale object.
    pub fn current_locale(&self) -> Option<&Locale> {
        self.locales.iter().find(|l| l.code == self.locale_code)
    }
}

/// A list of supported locales with a default.
#[derive(Clone)]
pub struct Locales {
    pub available: Vec<Locale>,
    pub default: Locale,
}

impl Locales {
    pub fn new(available: Vec<Locale>, default: Locale) -> Self {
        Self { available, default }
    }

    /// Simple convenience from code strings.
    pub fn from_codes(
        codes: &[&str],
        names: &[&str],
        default_code: &str,
    ) -> Self {
        let locales: Vec<Locale> = codes
            .iter()
            .zip(names.iter())
            .map(|(c, n)| Locale::new(c, n))
            .collect();
        let default = locales
            .iter()
            .find(|l| l.code == default_code)
            .cloned()
            .unwrap_or_else(|| Locale::new(default_code, default_code));
        Self {
            available: locales,
            default,
        }
    }

    /// Detect the best locale from an Accept-Language header.
    pub fn from_accept_language(&self, header: &str) -> &Locale {
        for part in header.split(',') {
            let lang =
                part.split(';').next().unwrap_or("").trim().to_lowercase();
            if let Some(locale) = self
                .available
                .iter()
                .find(|l| l.code.to_lowercase() == lang)
            {
                return locale;
            }
            if let Some(base) = lang.split('-').next() {
                if let Some(locale) = self
                    .available
                    .iter()
                    .find(|l| l.code.to_lowercase() == base)
                {
                    return locale;
                }
            }
        }
        &self.default
    }

    /// Detect locale from URL path prefix (e.g., "/fr/about" → "fr").
    pub fn from_url_path(&self, path: &str) -> &Locale {
        let segments: Vec<&str> =
            path.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(first) = segments.first() {
            if let Some(locale) =
                self.available.iter().find(|l| l.code == *first)
            {
                return locale;
            }
        }
        &self.default
    }
}

/// Resolve locale from HTTP request headers (SSR).
pub fn resolve_locale_from_request(
    headers: &HashMap<String, String>,
    cookie_name: &str,
    available: &[&str],
    default: &str,
) -> String {
    for (header, value) in headers {
        if header.to_lowercase() == "cookie" {
            for pair in value.split(';') {
                let pair = pair.trim();
                if let Some((k, v)) = pair.split_once('=') {
                    if k.trim() == cookie_name {
                        let locale = v.trim();
                        if available.contains(&locale) {
                            return locale.to_string();
                        }
                    }
                }
            }
        }
    }
    if let Some(al) = headers.get("accept-language") {
        for part in al.split(',') {
            let lang = part.split(';').next().unwrap_or("").trim();
            let base = lang.split('-').next().unwrap_or(lang);
            if available.contains(&base) {
                return base.to_string();
            }
            if available.contains(&lang) {
                return lang.to_string();
            }
        }
    }
    default.to_string()
}

/// Interpolate variables into a template string: `Hello, {name}!`
pub fn interpolate(template: &str, vars: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{key}}}"), value);
        result = result.replace(&format!("{{ {key} }}"), value);
    }
    result
}

/// A scoped wrapper around I18nContext that prefixes keys.
#[derive(Clone)]
pub struct ScopedContext {
    pub ctx: I18nContext,
    pub prefix: String,
}

impl ScopedContext {
    pub fn new(ctx: I18nContext, prefix: &str) -> Self {
        Self {
            ctx,
            prefix: prefix.to_string(),
        }
    }

    pub fn t(
        &self,
        key: &str,
        vars: &[(&str, &str)],
    ) -> Result<String, I18nError> {
        let full_key = if key.is_empty() {
            self.prefix.clone()
        } else {
            format!("{}.{}", self.prefix, key)
        };
        self.ctx.t(&full_key, vars)
    }

    pub fn set_locale(&mut self, code: &str) -> Result<(), I18nError> {
        self.ctx.set_locale(code)
    }

    pub fn current_locale(&self) -> Option<&Locale> {
        self.ctx.current_locale()
    }
}
