//! Invariant tests for montrs-i18n.

use montrs_i18n::*;
use std::collections::HashMap;

#[test]
fn test_locale_new_and_display() {
    let locale = Locale::new("en", "English");
    assert_eq!(locale.code, "en");
    assert_eq!(locale.display_name, "English");
    assert_eq!(locale.to_string(), "English (en)");
}

#[test]
fn test_locale_ordering() {
    let en = Locale::new("en", "English");
    let fr = Locale::new("fr", "French");
    assert!(en < fr);
}

#[test]
fn test_i18n_context_new() {
    let translations: LocaleTranslations = HashMap::new();
    let locales = vec![Locale::new("en", "English")];
    let ctx = I18nContext::new(translations, "en", locales);
    assert_eq!(ctx.locale_code, "en");
    assert!(ctx.keys().is_empty());
}

#[test]
fn test_i18n_context_set_locale() {
    let translations: LocaleTranslations = HashMap::new();
    let locales =
        vec![Locale::new("en", "English"), Locale::new("fr", "French")];
    let mut ctx = I18nContext::new(translations, "en", locales);
    ctx.set_locale("fr").unwrap();
    assert_eq!(ctx.locale_code, "fr");
}

#[test]
fn test_i18n_context_set_locale_invalid() {
    let translations: LocaleTranslations = HashMap::new();
    let locales = vec![Locale::new("en", "English")];
    let mut ctx = I18nContext::new(translations, "en", locales);
    assert!(ctx.set_locale("de").is_err());
}

#[test]
fn test_interpolate_simple() {
    let result = interpolate("Hello, {name}!", &[("name", "World")]);
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_interpolate_multiple() {
    let template = "You have {count} {items}";
    let result = interpolate(template, &[("count", "5"), ("items", "apples")]);
    assert_eq!(result, "You have 5 apples");
}

#[test]
fn test_translate_with_vars() {
    let mut en: TranslationMap = indexmap::IndexMap::new();
    en.insert("greeting".to_string(), "Hello, {name}!".to_string());
    let mut translations = HashMap::new();
    translations.insert("en".to_string(), en);

    let locales = vec![Locale::new("en", "English")];
    let ctx = I18nContext::new(translations, "en", locales);

    let result = ctx.t("greeting", &[("name", "Alice")]).unwrap();
    assert_eq!(result, "Hello, Alice!");
}

#[test]
fn test_translate_key_not_found() {
    let translations: LocaleTranslations = HashMap::new();
    let locales = vec![Locale::new("en", "English")];
    let ctx = I18nContext::new(translations, "en", locales);

    assert!(ctx.t("nonexistent", &[]).is_err());
}

#[test]
fn test_translate_fallback_to_default() {
    let mut en: TranslationMap = indexmap::IndexMap::new();
    en.insert("hello".to_string(), "Hello!".to_string());
    let mut translations = HashMap::new();
    translations.insert("en".to_string(), en);

    let locales =
        vec![Locale::new("en", "English"), Locale::new("fr", "French")];
    let mut ctx = I18nContext::new(translations, "en", locales);
    ctx.set_locale("fr").unwrap();

    // Falls back to default locale (en) since fr has no translations.
    let result = ctx.t("hello", &[]).unwrap();
    assert_eq!(result, "Hello!");
}

#[test]
fn test_pluralize() {
    let mut en: TranslationMap = indexmap::IndexMap::new();
    en.insert("items.one".to_string(), "{count} item".to_string());
    en.insert("items.other".to_string(), "{count} items".to_string());
    let mut translations = HashMap::new();
    translations.insert("en".to_string(), en);

    let locales = vec![Locale::new("en", "English")];
    let ctx = I18nContext::new(translations, "en", locales);

    let result = ctx.t_plural("items", 1, &[]).unwrap();
    assert_eq!(result, "1 item");
    let result = ctx.t_plural("items", 5, &[]).unwrap();
    assert_eq!(result, "5 items");
}

#[test]
fn test_locales_from_codes() {
    let locales =
        Locales::from_codes(&["en", "fr"], &["English", "French"], "en");
    assert_eq!(locales.default.code, "en");
    assert_eq!(locales.available.len(), 2);
}

#[test]
fn test_locales_from_accept_language() {
    let locales = Locales::from_codes(
        &["en", "fr", "ar"],
        &["English", "French", "Arabic"],
        "en",
    );
    let detected = locales.from_accept_language("fr-FR,fr;q=0.9,en;q=0.8");
    assert_eq!(detected.code, "fr");
}

#[test]
fn test_locales_from_url_path() {
    let locales =
        Locales::from_codes(&["en", "fr"], &["English", "French"], "en");
    let detected = locales.from_url_path("/fr/about/page");
    assert_eq!(detected.code, "fr");
}

#[test]
fn test_resolve_locale_from_request_cookie() {
    let mut headers = HashMap::new();
    headers.insert("cookie".to_string(), "locale=fr; session=abc".to_string());
    let result =
        resolve_locale_from_request(&headers, "locale", &["en", "fr"], "en");
    assert_eq!(result, "fr");
}

#[test]
fn test_scoped_context() {
    let mut en: TranslationMap = indexmap::IndexMap::new();
    en.insert("auth.login".to_string(), "Sign in".to_string());
    en.insert("auth.logout".to_string(), "Sign out".to_string());
    let mut translations = HashMap::new();
    translations.insert("en".to_string(), en);

    let locales = vec![Locale::new("en", "English")];
    let ctx = I18nContext::new(translations, "en", locales);
    let scoped = ScopedContext::new(ctx, "auth");

    let result = scoped.t("login", &[]).unwrap();
    assert_eq!(result, "Sign in");
}

#[test]
fn test_from_dir() {
    let dir = tempfile::tempdir().unwrap();
    let en_file = dir.path().join("en.json");
    std::fs::write(&en_file, r#"{"hello": "Hello!", "bye": "Goodbye!"}"#)
        .unwrap();

    let ctx = I18nContext::from_dir(dir.path(), "en", &["en"], "json").unwrap();
    assert_eq!(ctx.t("hello", &[]).unwrap(), "Hello!");
    assert_eq!(ctx.t("bye", &[]).unwrap(), "Goodbye!");
    assert_eq!(ctx.keys().len(), 2);
}

#[test]
fn test_error_display() {
    let err = I18nError::LocaleNotFound("xx".to_string());
    assert!(err.to_string().contains("xx"));
    let err = I18nError::KeyNotFound("key".to_string());
    assert!(err.to_string().contains("key"));
    let err = I18nError::MissingVar("var".to_string());
    assert!(err.to_string().contains("var"));
}
