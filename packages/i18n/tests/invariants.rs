//! Invariant tests for montrs-i18n.

use montrs_i18n::{prelude::*, *};

// Generate the test locale module once.
declare_locales! {
    path: "locales",
    default: "en",
    locales: ["en", "fr", "ar"],
    en: { hello: "Hello", bye: "Goodbye" },
    fr: { hello: "Bonjour", bye: "Au revoir" },
    ar: { hello: "As-salamu alaykum", bye: "Maa as-salaama" },
}

#[test]
fn test_direction_display() {
    assert_eq!(Direction::LeftToRight.as_str(), "ltr");
    assert_eq!(Direction::RightToLeft.as_str(), "rtl");
    assert_eq!(Direction::Auto.as_str(), "auto");
}

#[test]
fn test_locale_enum() {
    assert_eq!(i18n::Locale::en.as_str(), "en");
    assert_eq!(i18n::Locale::fr.as_str(), "fr");
    assert_eq!(i18n::Locale::ar.as_str(), "ar");
    assert_eq!(i18n::Locale::ALL.len(), 3);
    assert_eq!(i18n::Locale::default(), i18n::Locale::en);
}

#[test]
fn test_locale_parse() {
    let parsed: i18n::Locale = "fr".parse().unwrap();
    assert_eq!(parsed, i18n::Locale::fr);
    assert!("xx".parse::<i18n::Locale>().is_err());
}

#[test]
fn test_locale_direction() {
    assert_eq!(i18n::Locale::en.direction(), Direction::LeftToRight);
}

#[test]
fn test_locale_get_all() {
    let all = i18n::Locale::get_all();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_plural_module() {
    assert_eq!(plural::get_plural_category(0), "zero");
    assert_eq!(plural::get_plural_category(1), "one");
    assert_eq!(plural::get_plural_category(5), "few");
    assert_eq!(plural::get_plural_category(100), "other");
}

#[test]
fn test_formatting() {
    let n = formatting::number(42.5, "en");
    assert!(n.contains("42"));
    let l = formatting::list(&["a", "b", "c"], "en");
    assert_eq!(l, "a, b, c");
}

#[test]
fn test_macro_helpers() {
    use montrs_i18n::__private::*;
    let w = LitWrapper::new("hello");
    assert_eq!(w.inner(), "hello");
    assert_eq!(format!("{}", w), "hello");
    assert_eq!(get_key_component("foo.bar"), "foo.bar");
}

#[test]
fn test_scoped_locale() {
    let s = i18n::ScopedLocale::new(i18n::Locale::en);
    assert_eq!(s.inner(), i18n::Locale::en);
    assert_eq!(s.as_str(), "en");
    let s2: i18n::ScopedLocale<i18n::Locale> =
        ScopedLocale::new(i18n::Locale::fr);
    assert_eq!(s2.as_str(), "fr");
}

#[test]
fn test_translation_unit_id() {
    use montrs_i18n::locale_traits::TranslationUnitId;
    assert!(().to_str().is_none());
}

#[test]
fn test_locale_impls() {
    let loc = i18n::Locale::en;
    assert_eq!(loc.as_ref() as &str, "en");
    assert_eq!(loc.to_string(), "en");
    assert_eq!(serde_json::to_string(&loc).unwrap(), "\"en\"");
    let deser: i18n::Locale = serde_json::from_str("\"fr\"").unwrap();
    assert_eq!(deser, i18n::Locale::fr);
}
