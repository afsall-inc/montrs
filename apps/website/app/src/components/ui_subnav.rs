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

//! Persistent sub-navigation for the `/ui` section: breadcrumbs + one-click
//! links to every UI subpage, so you never need the header dropdown.

use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

const LINKS: &[(&str, &str)] = &[
    ("/ui", "MontRS UI"),
    ("/ui/components", "Components"),
    ("/ui/blocks", "Blocks"),
    ("/ui/icons", "Icons"),
    ("/ui/motion", "Motion"),
    ("/ui/themes", "Themes"),
    ("/ui/backgrounds", "Backgrounds"),
];

/// Breadcrumb label for the current path (falls back to "UI").
fn crumb_label(path: &str) -> &'static str {
    LINKS
        .iter()
        .find(|(href, _)| *href == path)
        .map(|(_, label)| *label)
        .unwrap_or("UI")
}

#[component]
pub fn UiSubNav() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let path = Signal::derive(move || location.pathname.get());

    // Only render inside the /ui section.
    let visible = move || path.get().starts_with("/ui");

    let crumb = move || crumb_label(&path.get());

    view! {
        <Show when=move || visible()>
            <nav
                class="sticky top-16 z-30 border-b border-border bg-background/80 backdrop-blur"
                aria-label="UI sections"
            >
                <div class="page-container flex flex-col gap-2 py-2">
                    <ol class="flex items-center gap-1.5 text-xs text-muted-foreground">
                        <li>
                            <a
                                href="/"
                                class="transition-colors hover:text-foreground"
                                on:click={
                                    let nav = navigate.clone();
                                    move |ev| {
                                        ev.prevent_default();
                                        nav("/", Default::default());
                                    }
                                }
                            >"Home"</a>
                        </li>
                        <li aria-hidden="true">"/"</li>
                        <li>
                            <a
                                href="/ui"
                                class="transition-colors hover:text-foreground"
                                on:click={
                                    let nav = navigate.clone();
                                    move |ev| {
                                        ev.prevent_default();
                                        nav("/ui", Default::default());
                                    }
                                }
                            >"UI"</a>
                        </li>
                        {move || {
                            let c = crumb();
                            if c == "UI" {
                                None
                            } else {
                                Some(view! {
                                    <>
                                        <li aria-hidden="true">"/"</li>
                                        <li class="text-foreground">{c}</li>
                                    </>
                                }.into_any())
                            }
                        }}
                    </ol>

                    <div class="flex flex-wrap items-center gap-1">
                        {LINKS.iter().copied().map(|(href, label)| {
                            let nav = navigate.clone();
                            let is_active = move || path.get() == href;
                            view! {
                                <a
                                    href=href
                                    class=move || {
                                        let base = "rounded-md px-2.5 py-1.5 text-sm transition-colors";
                                        if is_active() {
                                            format!("{base} bg-accent font-medium text-foreground")
                                        } else {
                                            format!("{base} text-muted-foreground hover:bg-accent/60 hover:text-foreground")
                                        }
                                    }
                                    on:click=move |ev| {
                                        ev.prevent_default();
                                        nav(href, Default::default());
                                    }
                                >{label}</a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </nav>
        </Show>
    }
}
