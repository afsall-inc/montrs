//! I18nContext — reactive locale state and translation access.

use crate::{
    fetch_locale::{self, signal_maybe_once_then},
    locale_traits::*,
    scopes::Scope,
};
use leptos::prelude::*;
use leptos_meta::Html;
use std::borrow::Cow;

const COOKIE_PREFERRED_LANG: &str = "montrs_pref_locale";

/// The heart of the i18n system. A reactive signal to the current locale.
#[derive(Debug)]
pub struct I18nContext<L: Locale> {
    locale_signal: RwSignal<L>,
}

impl<L: Locale> Clone for I18nContext<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Locale> Copy for I18nContext<L> {}

impl<L: Locale> I18nContext<L> {
    pub fn get_locale(self) -> L {
        self.locale_signal.get()
    }
    pub fn get_locale_untracked(self) -> L {
        self.locale_signal.get_untracked()
    }
    pub fn get_keys(self) -> L::Keys {
        LocaleKeys::from_locale(self.get_locale())
    }
    pub fn get_keys_untracked(self) -> L::Keys {
        LocaleKeys::from_locale(self.get_locale_untracked())
    }
    pub fn set_locale(self, lang: L) {
        self.locale_signal.set(lang);
    }
    pub fn set_locale_untracked(self, lang: L) {
        *self.locale_signal.write_untracked() = lang;
    }
    pub fn scope<NS: Scope<L>>(self) -> I18nContext<L> {
        self
    }
    pub(crate) fn from_context() -> Option<Self> {
        use_context()
    }
    pub(crate) fn provide(this: Self) {
        provide_context(this);
    }
}

/// Options to init or provide an I18nContext.
pub struct I18nContextOptions<'a, L: Locale> {
    pub enable_cookie: bool,
    pub cookie_name: Cow<'a, str>,
    pub _marker: std::marker::PhantomData<L>,
}

impl<L: Locale> Default for I18nContextOptions<'_, L> {
    fn default() -> Self {
        Self {
            enable_cookie: false,
            cookie_name: Cow::Borrowed(COOKIE_PREFERRED_LANG),
            _marker: std::marker::PhantomData,
        }
    }
}

fn init_context_inner<L: Locale>(
    set_lang_cookie: WriteSignal<Option<L>>,
    initial_locale: Memo<L>,
) -> I18nContext<L> {
    let locale_signal = RwSignal::new(initial_locale.get_untracked());
    Effect::new(move |_| {
        locale_signal.set(initial_locale.get());
    });
    Effect::new_isomorphic(move |_| {
        set_lang_cookie.set(Some(locale_signal.get()));
    });
    I18nContext { locale_signal }
}

pub fn init_i18n_context_with_options<L: Locale>(
    _options: I18nContextOptions<L>,
) -> I18nContext<L> {
    let (lang_cookie, set_lang_cookie) = signal::<Option<L>>(None);
    let initial: L = lang_cookie.get_untracked().unwrap_or_default();
    let memo = Memo::new(move |_| initial);
    init_context_inner(set_lang_cookie, memo)
}

pub fn init_i18n_context<L: Locale>() -> I18nContext<L> {
    init_i18n_context_with_options(Default::default())
}

pub fn provide_i18n_context<L: Locale>() -> I18nContext<L> {
    leptos_meta::provide_meta_context();
    I18nContext::from_context().unwrap_or_else(|| {
        let ctx = init_i18n_context_with_options(Default::default());
        I18nContext::provide(ctx);
        ctx
    })
}

pub fn use_i18n_context<L: Locale>() -> I18nContext<L> {
    I18nContext::from_context().expect("MontRS I18n context is missing")
}

pub fn use_i18n_with_scope<L: Locale>() -> I18nContext<L> {
    use_i18n_context::<L>()
}

fn derive_initial_locale_signal<L: Locale>(
    init: Option<Signal<L>>,
) -> Signal<Option<L>> {
    init.map(|s| Signal::derive(move || Some(s.get())))
        .unwrap_or_default()
}

pub fn init_i18n_subcontext_with_options<L: Locale>(
    initial_locale: Option<Signal<L>>,
    _cookie_name: Option<Cow<str>>,
    _cookie_options: Option<()>,
    _ssr_lang_header: Option<()>,
) -> I18nContext<L> {
    let initial_locale = derive_initial_locale_signal(initial_locale);
    let (lang_cookie, set_lang_cookie) = signal(None);
    let parent =
        I18nContext::<L>::from_context().map(|c| c.get_locale_untracked());
    let fetch_memo = fetch_locale::fetch_locale(None);
    let parent = signal_maybe_once_then(parent, fetch_memo);
    let listener = Memo::new(move |prev| {
        let cookie: Option<L> = lang_cookie.get_untracked();
        let p = parent.get();
        if prev.is_none() {
            cookie.or(initial_locale.get()).unwrap_or(p)
        } else {
            initial_locale.get().or(cookie).unwrap_or(p)
        }
    });
    init_context_inner(set_lang_cookie, listener)
}

pub fn init_i18n_subcontext<L: Locale>(
    initial_locale: Option<Signal<L>>,
) -> I18nContext<L> {
    init_i18n_subcontext_with_options::<L>(initial_locale, None, None, None)
}

pub fn provide_i18n_context_component<L: Locale, Chil>(
    set_lang_attr: Option<bool>,
    set_dir_attr: Option<bool>,
    _enable_cookie: Option<bool>,
    _cookie_name: Option<Cow<str>>,
    _cookie_options: Option<()>,
    _ssr_lang_header: Option<()>,
    children: impl FnOnce() -> Chil + Send,
) -> impl IntoView
where
    Chil: IntoView,
{
    let i18n = provide_i18n_context::<L>();
    let lang = set_lang_attr
        .unwrap_or(true)
        .then(|| move || i18n.get_locale().as_str());
    let dir = set_dir_attr
        .unwrap_or(true)
        .then(|| move || i18n.get_locale().direction().as_str());
    view! {
        {children()}
        <Html attr:lang=lang attr:dir=dir />
    }
}
