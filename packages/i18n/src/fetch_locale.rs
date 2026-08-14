use crate::locale_traits::Locale;
use leptos::prelude::*;

/// Fetch locale from defaults.
pub fn fetch_locale<L: Locale>(cookie: Option<L>) -> Memo<L> {
    let locale: L = cookie.unwrap_or_default();
    Memo::new(move |_| locale)
}

/// Create a signal that fires once from parent, then from a secondary source.
pub fn signal_maybe_once_then<L: Locale>(
    parent: Option<L>,
    secondary: Memo<L>,
) -> Memo<L> {
    if let Some(p) = parent {
        Memo::new(move |_| p)
    } else {
        secondary
    }
}
