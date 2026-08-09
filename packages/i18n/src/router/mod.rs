//! Router integration — locale-prefixed routes.

use crate::locale_traits::Locale;
use leptos::prelude::*;

/// Generate a localized href with the current locale.
pub fn localized_href<L: Locale>(
    ctx: crate::I18nContext<L>,
    path: &str,
) -> String {
    let locale = ctx.get_locale().as_str();
    format!("/{}/{}", locale, path.trim_start_matches('/'))
}

/// Sync the locale from the current URL path.
pub fn use_locale_from_url<L: Locale>(
    ctx: crate::I18nContext<L>,
    available: &'static [L],
) {
    let loc = leptos_router::hooks::use_location();
    Effect::new(move || {
        let path = loc.pathname.get();
        let segments: Vec<&str> =
            path.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(first) = segments.first() {
            if let Some(locale) =
                available.iter().find(|l| l.as_str() == *first)
            {
                ctx.set_locale(*locale);
            }
        }
    });
}
