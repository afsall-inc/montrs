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

#![allow(dead_code)]

//! MontRS internationalization — reactive translations for MontRS applications.
//!
//! Provides locale detection, pluralization, interpolation, scoping,
//! formatting, and signal-powered locale switching for MontRS apps.
//!
//! # Key features
//! - `declare_locales!` macro for compile-time locale definition
//! - `I18nContext<RwSignal<Locale>>` for reactive locale switching
//! - `t!`, `td!`, `tu!` macros for translations
//! - `t_plural!`, `t_format!` macros for pluralization + formatting
//! - Scoping (`use_i18n_scoped!`, `scope_i18n!`)
//! - Cookie-based locale persistence
//! - SSR `Accept-Language` detection
//!
//! # Example
//! ```rust,ignore
//! use montrs_i18n::prelude::*;
//! use montrs_i18n::declare_locales;
//!
//! declare_locales! {
//!     path: "locales",
//!     default: "en",
//!     locales: ["en", "fr"],
//!     en: {
//!         hello: "Hello!",
//!         click_count: "You clicked {{ count }} times",
//!     },
//!     fr: {
//!         hello: "Bonjour!",
//!         click_count: "Vous avez cliqué {{ count }} fois",
//!     },
//! }
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <I18nContextProvider>
//!             <Home />
//!         </I18nContextProvider>
//!     }
//! }
//! ```

mod context;
mod display;
mod fetch_locale;
mod locale;
pub mod locale_traits;
mod macro_helpers;
mod macros;
mod scopes;

pub mod formatting;
pub mod plural;
pub mod router;

pub use context::{
    I18nContext, I18nContextOptions, provide_i18n_context, use_i18n_context,
    use_i18n_with_scope,
};
pub use display::LangDisplay;
pub use locale_traits::{Direction, Locale, LocaleKeys};
pub use scopes::ScopedLocale;

#[doc(hidden)]
pub mod __private {
    pub use crate::{locale_traits::TranslationUnitId, macro_helpers::*};

    pub trait AnyBound {}
    impl<T: ?Sized> AnyBound for T {}
}

pub mod prelude {
    pub use crate::{
        Direction, I18nContext, I18nContextOptions, LangDisplay, Locale,
        LocaleKeys, ScopedLocale, define_scope, formatting, plural, router,
        scope_i18n, scope_locale, t, t_display, t_format, t_format_display,
        t_format_string, t_plural, t_plural_ordinal, t_string, td, td_display,
        td_format, td_format_display, td_format_string, td_plural,
        td_plural_ordinal, td_string, tu, tu_display, tu_format,
        tu_format_display, tu_format_string, tu_plural, tu_plural_ordinal,
        tu_string, use_i18n_context, use_i18n_scoped, use_i18n_with_scope,
    };
}
