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
//! `NavLink` prevents the default and drives `use_navigate` directly. The
//! framework-wide companion — `montrs_core::RouterAnchorGuard`, which applies
//! the same treatment to every internal anchor and neutralises placeholder
//! hash links — is auto-installed by `RouterOutlet`, so apps don't add it.

use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

/// An internal link that performs SPA navigation via `use_navigate`.
#[component]
pub fn NavLink(
    #[prop(into)] href: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let href_attr = href.clone();
    let href_nav = href;
    let active_href = href_attr.clone();
    // A `Memo` gives the reactive read a tracking owner, so the current route
    // is tracked (and the SSR "outside a reactive context" warning is gone).
    let is_active = Memo::new(move |_| {
        let current = location.pathname.get();
        current == active_href
            || (active_href != "/"
                && current.starts_with(&format!("{}/", active_href)))
    });
    view! {
        <a
            href=href_attr
            class=class
            aria-current=move || is_active.get().then_some("page")
            on:click=move |ev| {
                ev.prevent_default();
                navigate(&href_nav, Default::default());
            }
        >
            {children()}
        </a>
    }
}
