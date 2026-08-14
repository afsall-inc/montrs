# montrs-i18n — Invariants

## 1. Responsibility
Provide runtime internationalization: locale management, translation lookups, pluralization, interpolation, formatting, and scoping.

## 2. Invariants
- **Compile-time + runtime**: `declare_locales!` macro for compile-time locale definitions, plus runtime file-based loading.
- **Fallback chain**: Missing translations fall back to the default locale, then to the key itself.
- **Pluralization uses CLDR categories**: zero, one, two, few, many, other via `t_plural!` / `t_plural_ordinal!`.
- **Interpolation uses `{{key}}` syntax**: Double-brace template replacement.
- **SSR-aware**: `Accept-Language` header detection, cookie-based locale persistence.
- **Reactive**: `I18nContext<RwSignal<Locale>>` for signal-powered locale switching.
- **Scoping**: `use_i18n_scoped!`, `scope_i18n!`, `scope_locale!`, `define_scope!` for namespaced keys.
- **Formatting**: Number, currency, date, time, datetime, list formatting via `t_format!`.
- **Router integration**: `I18nRoute` for locale-prefixed routing.
- **Leptos integration**: `I18nContextProvider`, `I18nSubContextProvider` components.

## 3. Boundary
- **In-Scope**: Locale trait, translation loading, key lookup, interpolation, pluralization, scoping, formatting, locale resolution, router integration.
- **Out-of-Scope**: ICU4X integration, machine translation, translation file management tools.

## 4. Agent Guidelines
- Use `declare_locales!` for compile-time locale definitions.
- Use `I18nContext::from_dir()` for runtime file-based loading.
- Use `t!()`, `t_plural!()`, `t_format!()` macros for translations.
- Use `use_i18n_scoped!()` for namespaced keys.
- Use `I18nContextProvider` component to wrap the app.
- The auth i18n plugin (`montrs-auth::plugins::I18nPlugin`) provides error code → message mappings.