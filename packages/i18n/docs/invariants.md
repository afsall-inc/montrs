# montrs-i18n — Invariants

## 1. Responsibility
Provide runtime internationalization: locale management, translation lookups, pluralization, and interpolation.

## 2. Invariants
- **No compile-time codegen**: Translations are loaded at runtime from JSON/TOML/YAML files.
- **Fallback chain**: Missing translations fall back to the default locale's entry, then to the key itself.
- **Pluralization uses CLDR categories**: zero, one, two, few, many, other.
- **Interpolation uses `{key}` syntax**: No custom template engine — simple string replacement.
- **SSR-aware**: `resolve_locale_from_request` reads cookies and Accept-Language headers.

## 3. Boundary
- **In-Scope**: Locale enum, translation loading, key lookup, interpolation, pluralization, scoping, locale resolution.
- **Out-of-Scope**: Compile-time validation of translation files, ICU integration, date/number formatting.

## 4. Agent Guidelines
- Use `I18nContext::from_dir()` for file-based loading.
- Use `ctx.t("key", &[("var", "val")])` for translations with interpolation.
- Use `ctx.t_plural("key", count, &[])` for pluralization.
- Use `ScopedContext::new(ctx, "prefix")` for namespaced keys.