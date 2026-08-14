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
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(load_theme_preference());

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

fn load_theme_preference() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) =
            web_sys::window().and_then(|w| w.local_storage().ok()?)
        {
            if let Ok(Some(value)) = storage.get_item("montrs-theme") {
                match value.as_str() {
                    "light" => return ThemeMode::Light,
                    "dark" => return ThemeMode::Dark,
                    _ => {}
                }
            }
        }
    }
    ThemeMode::System
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
