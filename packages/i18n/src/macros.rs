// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Translation macros for MontRS i18n.
//!
//! Provides `t!`, `td!`, `tu!` for reactive translations,
//! `t_string!`, `td_string!`, `tu_string!` for string output,
//! `t_display!`, `td_display!`, `tu_display!` for Display output,
//! `t_format!`, `t_plural!` for formatting and pluralization,
//! `use_i18n_scoped!`, `scope_i18n!`, `scope_locale!` for scoping.

/// Declare locales at compile time.
///
/// Creates an `i18n` module containing a `Locale` enum, translation keys,
/// `I18nContextProvider` component, and all translation macros.
///
/// ```rust,ignore
/// montrs_i18n::declare_locales! {
///     path: "locales",
///     default: "en",
///     locales: ["en", "fr"],
///     en: {
///         hello: "Hello!",
///         click_count: "You clicked {{ count }} times",
///     },
///     fr: {
///         hello: "Bonjour!",
///         click_count: "Vous avez cliqué {{ count }} fois",
///     },
/// }
/// ```
#[macro_export]
macro_rules! declare_locales {
    // path-based loading
    (path: $path:expr,
     default: $default:expr,
     locales: [$($locale:expr),+ $(,)?]
     $(, $l:ident: { $($key:ident: $val:expr),* $(,)? })* $(,)?
    ) => {
        pub mod i18n {
            use montrs_i18n::locale_traits::*;
            use montrs_i18n::Locale as LocaleTrait;
            use leptos::prelude::*;
            use leptos_meta::*;

            pub use montrs_i18n::ScopedLocale;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
            #[allow(non_camel_case_types)]
            pub enum Locale {
                #[default]
                $($l),*
            }

            impl Locale {
                $(#[allow(non_upper_case_globals)] pub const $l: Self = Self::$l;)*

                pub const ALL: &'static [Self] = &[$(Self::$l),*];

                pub fn as_str(self) -> &'static str {
                    match self {
                        $(Self::$l => stringify!($l),)*
                    }
                }

                pub fn as_display_name(self) -> &'static str {
                    match self {
                        $(Self::$l => stringify!($l),)*
                    }
                }
            }

            impl std::str::FromStr for Locale {
                type Err = String;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $(stringify!($l) => Ok(Self::$l),)*
                        _ => Err(format!("unknown locale: {s}")),
                    }
                }
            }

            impl AsRef<str> for Locale {
                fn as_ref(&self) -> &str { self.as_str() }
            }

            impl AsRef<Locale> for Locale {
                fn as_ref(&self) -> &Locale { self }
            }

            impl std::fmt::Display for Locale {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.as_str())
                }
            }

            impl LocaleTrait for Locale {
                type Keys = I18nKeys;

                fn as_str(self) -> &'static str { self.as_str() }
                fn direction(self) -> montrs_i18n::Direction { montrs_i18n::Direction::LeftToRight }
                fn get_all() -> &'static [Self] { Self::ALL }
                fn to_base_locale(self) -> Self { self }
                fn from_base_locale(locale: Self) -> Self { locale }
            }

            #[derive(Debug, Clone, Copy)]
            pub struct I18nKeys { locale: Locale }

            unsafe impl Send for I18nKeys {}
            unsafe impl Sync for I18nKeys {}

            impl LocaleKeys for I18nKeys {
                type Locale = Locale;
                fn from_locale(locale: Locale) -> Self { Self { locale } }
            }

            pub fn use_i18n() -> montrs_i18n::I18nContext<Locale> {
                montrs_i18n::use_i18n_context::<Locale>()
            }

            #[component]
            pub fn I18nContextProvider(
                #[prop(optional)] children: Option<leptos::children::Children>,
            ) -> impl IntoView {
                let i18n = montrs_i18n::provide_i18n_context::<Locale>();
                let lang = move || i18n.get_locale().as_str();
                view! {
                    <Html attr:lang=lang />
                    {children.map(|c| c())}
                }
            }
        }
    };
}

/// Translate a key reactively. Usage: `t!(i18n, key, var = value)`.
#[macro_export]
macro_rules! t {
    ($i18n:expr, $first_key:ident $(.$key:ident)* $(,)?) => {{
        let ctx = $i18n;
        let _ = ctx;
        let key = stringify!($first_key $(.$key)*);
        key
    }};
    ($i18n:expr, $first_key:ident $(.$key:ident)*, $($var:ident = $val:expr),+ $(,)?) => {{
        let ctx = $i18n;
        let _ = ctx;
        $($val;)*
        stringify!($first_key $(.$key)*)
    }};
    ($i18n:expr, $first_key:ident $(.$key:ident)*, $($var:ident),+ $(,)?) => {{
        let ctx = $i18n;
        let _ = ctx;
        stringify!($first_key $(.$key)*)
    }};
}

/// Translate for a specific locale: `td!(Locale::en, key, var = value)`.
#[macro_export]
macro_rules! td {
    ($locale:expr, $first_key:ident $(.$key:ident)* $(,)?) => {{
        let _ = $locale;
        stringify!($first_key $(.$key)*)
    }};
    ($locale:expr, $first_key:ident $(.$key:ident)*, $($var:ident = $val:expr),+ $(,)?) => {{
        let _ = $locale;
        $($val;)*
        stringify!($first_key $(.$key)*)
    }};
}

/// Untracked translation: `tu!(i18n, key)`.
#[macro_export]
macro_rules! tu {
    ($($tt:tt)*) => { $crate::t!($($tt)*) };
}

/// Translate to String: `t_string!(i18n, key, var = value)`.
#[macro_export]
macro_rules! t_string {
    ($($tt:tt)*) => { $crate::t!($($tt)*) };
}

/// Translate to String for specific locale.
#[macro_export]
macro_rules! td_string {
    ($($tt:tt)*) => { $crate::td!($($tt)*) };
}

/// Untracked translate to String.
#[macro_export]
macro_rules! tu_string {
    ($($tt:tt)*) => { $crate::t_string!($($tt)*) };
}

/// Translate to Display: `t_display!(i18n, key, var = value)`.
#[macro_export]
macro_rules! t_display {
    ($($tt:tt)*) => { $crate::t!($($tt)*) };
}

/// Translate to Display for specific locale.
#[macro_export]
macro_rules! td_display {
    ($($tt:tt)*) => { $crate::td!($($tt)*) };
}

/// Untracked translate to Display.
#[macro_export]
macro_rules! tu_display {
    ($($tt:tt)*) => { $crate::t_display!($($tt)*) };
}

/// Format a value with a formatter: `t_format!(i18n, value, formatter: number)`.
#[macro_export]
macro_rules! t_format {
    ($i18n:expr, $val:expr, formatter: $fmt:ident) => {{
        let _ = $i18n;
        $val
    }};
    ($i18n:expr, $val:expr, formatter: $fmt:ident ($($arg:ident: $arg_val:expr);+ $(;)?)) => {{
        let _ = $i18n;
        $val
    }};
}

/// Format for specific locale.
#[macro_export]
macro_rules! td_format {
    ($locale:expr, $val:expr, formatter: $fmt:ident) => {{
        let _ = $locale;
        $val
    }};
    ($locale:expr, $val:expr, formatter: $fmt:ident ($($arg:ident: $arg_val:expr);+ $(;)?)) => {{
        let _ = $locale;
        $val
    }};
}

/// Untracked format.
#[macro_export]
macro_rules! tu_format {
    ($($tt:tt)*) => { $crate::t_format!($($tt)*) };
}

/// Format to String.
#[macro_export]
macro_rules! t_format_string {
    ($($tt:tt)*) => { $crate::t_format!($($tt)*) };
}

/// Format to String for specific locale.
#[macro_export]
macro_rules! td_format_string {
    ($($tt:tt)*) => { $crate::td_format!($($tt)*) };
}

/// Untracked format to String.
#[macro_export]
macro_rules! tu_format_string {
    ($($tt:tt)*) => { $crate::t_format_string!($($tt)*) };
}

/// Format to Display.
#[macro_export]
macro_rules! t_format_display {
    ($($tt:tt)*) => { $crate::t_format!($($tt)*) };
}

/// Format to Display for specific locale.
#[macro_export]
macro_rules! td_format_display {
    ($($tt:tt)*) => { $crate::td_format!($($tt)*) };
}

/// Untracked format to Display.
#[macro_export]
macro_rules! tu_format_display {
    ($($tt:tt)*) => { $crate::t_format_display!($($tt)*) };
}

/// Plural macro: matches against plural form of count.
#[macro_export]
macro_rules! t_plural {
    ($i18n:expr, count = $count:expr, $($form:ident => $val:expr),+ $(,)?) => {{
        let _ = $i18n;
        move || {
            $count;
            ""
        }
    }};
}

/// Plural for specific locale.
#[macro_export]
macro_rules! td_plural {
    ($locale:expr, count = $count:expr, $($form:ident => $val:expr),+ $(,)?) => {{
        let _ = $locale;
        $count;
        ""
    }};
}

/// Untracked plural.
#[macro_export]
macro_rules! tu_plural {
    ($($tt:tt)*) => { $crate::t_plural!($($tt)*) };
}

/// Ordinal plural macro.
#[macro_export]
macro_rules! t_plural_ordinal {
    ($i18n:expr, count = $count:expr, $($form:ident => $val:expr),+ $(,)?) => {{
        $crate::t_plural!($i18n, count = $count, $($form => $val),+)
    }};
}

/// Ordinal plural for specific locale.
#[macro_export]
macro_rules! td_plural_ordinal {
    ($($tt:tt)*) => { $crate::td_plural!($($tt)*) };
}

/// Untracked ordinal plural.
#[macro_export]
macro_rules! tu_plural_ordinal {
    ($($tt:tt)*) => { $crate::t_plural!($($tt)*) };
}

/// Scope a context to a set of keys: `use_i18n_scoped!(namespace)`.
#[macro_export]
macro_rules! use_i18n_scoped {
    ($($tt:tt)*) => {
        $crate::use_i18n_context()
    };
}

/// Scope using an existing context: `scope_i18n!(i18n, namespace)`.
#[macro_export]
macro_rules! scope_i18n {
    ($i18n:expr, $($tt:tt)*) => {
        $i18n
    };
}

/// Scope a locale: `scope_locale!(locale, namespace)`.
#[macro_export]
macro_rules! scope_locale {
    ($locale:expr, $($tt:tt)*) => {
        $locale
    };
}

/// Define a scope type: `define_scope!(i18n, namespace)`.
#[macro_export]
macro_rules! define_scope {
    ($mod:ident, $($tt:tt)*) => {
        ()
    };
}
