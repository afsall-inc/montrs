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

//! SPA-safe internal navigation.
//!
//! `leptos_router`'s global anchor interceptor defers `history.pushState`
//! until the leptos `<Routes>` tree resolves — but this app renders through
//! MontRS's own `RouterOutlet`, which never resolves that tree. Plain
//! `<a href>` therefore leaves the address bar stale, and placeholder
//! `href="#"` links can even swap the page content.
//!
//! `NavLink` prevents the default and drives `use_navigate` directly.
//! `AnchorGuard` is a document-level capture listener that applies the same
//! treatment to every internal anchor and neutralises placeholder hash links.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// An internal link that performs SPA navigation via `use_navigate`.
#[component]
pub fn NavLink(
    #[prop(into)] href: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let navigate = use_navigate();
    let href_attr = href.clone();
    let href_nav = href;
    view! {
        <a
            href=href_attr
            class=class
            on:click=move |ev| {
                ev.prevent_default();
                navigate(&href_nav, Default::default());
            }
        >
            {children()}
        </a>
    }
}

/// Keeps navigation sane under the custom MontRS `RouterOutlet`:
///
/// * placeholder `href="#"` anchors are neutralised (the router would
///   otherwise treat them as a navigation to `/`);
/// * internal `href="/..."` anchors are intercepted and driven through
///   `use_navigate` so the address bar updates immediately instead of
///   waiting for a `<Routes>` tree that never resolves.
///
/// Registered as a capture-phase listener so it runs before the router's own
/// bubble-phase anchor interceptor. It deliberately does *not* stop
/// propagation: local `on:click` handlers (scroll-to, analytics, …) still run.
#[component]
pub fn AnchorGuard() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    let navigate = use_navigate();

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::{JsCast, prelude::Closure};

        let navigate = navigate.clone();
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(
            move |ev: web_sys::MouseEvent| {
                if ev.default_prevented() {
                    return;
                }
                let Some(target) = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                else {
                    return;
                };
                let Ok(Some(anchor)) = target.closest("a") else {
                    return;
                };
                let href = anchor.get_attribute("href").unwrap_or_default();

                if href.is_empty() || href == "#" || href.starts_with('#') {
                    ev.prevent_default();
                    return;
                }
                if !href.starts_with('/') || href.starts_with("//") {
                    return;
                }
                if ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key()
                {
                    return;
                }
                if anchor.get_attribute("target").is_some()
                    || anchor.get_attribute("download").is_some()
                {
                    return;
                }

                ev.prevent_default();
                navigate(&href, Default::default());
            },
        ));

        let _ = document.add_event_listener_with_callback_and_bool(
            "click",
            cb.as_ref().unchecked_ref(),
            true,
        );
        cb.forget();
    });

    view! {
        <span class="hidden" aria-hidden="true"></span>
    }
}
