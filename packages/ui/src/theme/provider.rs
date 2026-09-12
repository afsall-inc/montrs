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

use leptos::prelude::*;

/// Theme mode for the application.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        match self {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => {
                #[cfg(target_arch = "wasm32")]
                {
                    web_sys::window()
                        .and_then(|w| {
                            w.match_media("(prefers-color-scheme: dark)")
                                .ok()?
                        })
                        .map(|m| m.matches())
                        .unwrap_or(false)
                }
                #[cfg(not(target_arch = "wasm32"))]
                false
            }
        }
    }
}

/// Provides theme context and dark mode toggling.
///
/// Wraps the application and applies the `.dark` class to `<html>`.
/// Supports `localStorage` persistence for user preference.
///
/// The initial signal is always [`ThemeMode::System`] so SSR and hydration
/// render identical markup; the saved preference is applied in a
/// client-only effect. For a flash-free first paint the app should render
/// `<html class="dark">` and use a small pre-paint script that removes the
/// class when the resolved preference is light.
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(ThemeMode::System);

    #[cfg(target_arch = "wasm32")]
    let system_dark = RwSignal::new(system_prefers_dark());

    // Apply the persisted preference and follow live OS theme changes.
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use web_sys::wasm_bindgen::{JsCast, prelude::Closure};

        if let Some(saved) = load_theme_preference() {
            theme.set(saved);
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)")
        else {
            return;
        };
        let cb = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(
            move |_ev: web_sys::Event| {
                system_dark.set(system_prefers_dark());
            },
        ));
        let _ = query.set_onchange(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    });

    #[cfg(target_arch = "wasm32")]
    let is_dark = Memo::new(move |_| match theme.get() {
        ThemeMode::Dark => true,
        ThemeMode::Light => false,
        ThemeMode::System => system_dark.get(),
    });

    #[cfg(not(target_arch = "wasm32"))]
    let is_dark = Memo::new(move |_| theme.get().is_dark());

    Effect::new(move |_| {
        if let Some(document) = document()
            && let Some(html) = document.document_element()
        {
            if is_dark.get() {
                let _ = html.class_list().add_1("dark");
            } else {
                let _ = html.class_list().remove_1("dark");
            }
        }
    });

    Effect::new(move |_| {
        save_theme_preference(theme.get());
    });

    provide_context(theme);

    view! {
        {children()}
    }
}

/// Reads the current theme mode from the reactive context.
pub fn use_theme() -> RwSignal<ThemeMode> {
    use_context::<RwSignal<ThemeMode>>().expect("ThemeProvider not found")
}

/// Toggles between light/dark/system modes.
pub fn toggle_theme() {
    let theme = use_theme();
    theme.update(|t| {
        *t = match t {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::System,
            ThemeMode::System => ThemeMode::Light,
        }
    });
}

/// Returns the saved theme preference, if the user has explicitly chosen one.
#[cfg(target_arch = "wasm32")]
fn load_theme_preference() -> Option<ThemeMode> {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok()?)?;
    let value = storage.get_item("montrs-theme").ok()??;
    match value.as_str() {
        "light" => Some(ThemeMode::Light),
        "dark" => Some(ThemeMode::Dark),
        "system" => Some(ThemeMode::System),
        _ => None,
    }
}

/// Whether the operating system currently prefers a dark color scheme.
#[cfg(target_arch = "wasm32")]
fn system_prefers_dark() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok()?)
        .map(|m| m.matches())
        .unwrap_or(false)
}

#[allow(unused_variables)]
fn save_theme_preference(mode: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) =
            web_sys::window().and_then(|w| w.local_storage().ok()?)
        {
            let value = match mode {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::System => "system",
            };
            let _ = storage.set_item("montrs-theme", value);
        }
    }
}

fn document() -> Option<web_sys::Document> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()?.document()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
