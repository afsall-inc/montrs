# montrs-i18n — Agent Guide

## Overview
Provides runtime internationalization for MontRS applications. Load translations from files, detect the user's locale, and render translated strings with interpolation and pluralization.

## Key Concepts
- **I18nContext**: Holds loaded translations and the active locale.
- **Locales**: List of available locales with default, plus detection helpers.
- **ScopedContext**: Prefixes translation keys for modular organization.
- **Pluralization**: Uses `key.one`, `key.other`, `key.few` etc. with fallback to `key.other`.

## Agent Usage
- `I18nContext::from_dir("locales", "en", &["en", "fr"], "json")` to load from files
- `ctx.t("key", &[("var", "val")])` for translations with interpolation
- `ctx.t_plural("items", count, &[])` for pluralized translations
- `Locales::from_accept_language(header)` for SSR locale detection
- `Locales::from_url_path("/fr/about")` for URL-based detection

## Local Invariants
Read `docs/invariants.md` before modifying.