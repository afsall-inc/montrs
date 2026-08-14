# montrs-i18n

Internationalization for MontRS. Runtime translation loading, locale detection, pluralization, and scoped key namespaces.

## Features

- **Runtime translation loading** from JSON and TOML files
- **Locale detection** from Accept-Language header, URL path prefix, or cookies
- **Variable interpolation**: `{name}` in translation templates
- **Pluralization**: zero, one, few, many, other forms
- **Scoping**: prefix-based key namespaces for modular translations
- **SSR language negotiation**: resolve locale from HTTP headers

## Usage

```rust
use montrs_i18n::{I18nContext, Locale, Locales};

// Load translations from a directory of JSON files
let ctx = I18nContext::from_dir("locales", "en", &["en", "fr", "ar"], "json")?;

// Simple translation
let greeting = ctx.t("hello", &[])?;

// With interpolation
let msg = ctx.t("click_count", &[("count", "5")])?;

// Pluralization
let items = ctx.t_plural("items", 2, &[])?;
```

## File Format

```json
{
  "hello": "Hello!",
  "greeting": "Hello, {name}!",
  "items.one": "{count} item",
  "items.other": "{count} items"
}
```