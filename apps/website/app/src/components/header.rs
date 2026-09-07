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

use crate::components::theme_customizer::ThemeCustomizer;
use leptos::prelude::*;
use montrs_core::nav::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

// NOTE: nav links use plain anchors with a `use_navigate` click handler.
// `use_navigate` (RouterContext::navigate) updates the internal location AND
// completes the browser-history navigation immediately — unlike `<A>`, whose
// global anchor interception defers the URL update until the leptos
// `<Routes>` tree resolves. Our custom RouterOutlet never resolves routes,
// so `<A>` would stop updating the address bar entirely.

#[component]
pub fn Header() -> impl IntoView {
    let theme = use_theme();
    let mobile_open = RwSignal::new(false);
    let navigate = use_navigate();

    let nav_links = [
        ("/auth", "Auth"),
        ("/runtime", "Runtime"),
        ("/ai", "AI Kit"),
        ("/orm", "ORM"),
        ("/ui/motion", "Motion"),
        ("/templates", "Templates"),
        ("/docs", "Docs"),
    ];

    let ui_links = [
        ("/ui", "MontRS UI"),
        ("/ui/components", "Components"),
        ("/ui/blocks", "Blocks"),
        ("/ui/icons", "Icons"),
        ("/ui/themes", "Themes"),
        ("/ui/backgrounds", "Backgrounds"),
    ];

    let ui_open = RwSignal::new(false);
    let customize_open = RwSignal::new(false);
    let search_q = RwSignal::new(String::new());
    let search_ref: NodeRef<leptos::html::Input> = NodeRef::new();
    let search_nav = navigate.clone();

    let on_search_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            let q = search_q.get().trim().to_string();
            if !q.is_empty() {
                search_nav(
                    &format!("/ui/icons?q={}", url_enc(&q)),
                    Default::default(),
                );
                search_q.set(String::new());
            }
        }
    };

    // "C" opens/closes the theme customizer (shark-ui style).
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::{JsCast, prelude::Closure};
        let Some(window) = web_sys::window() else {
            return;
        };
        let cb = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::wrap(Box::new(
            move |ev: web_sys::KeyboardEvent| {
                let key = ev.key();
                let in_field = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                    .is_some_and(|el| {
                        el.tag_name() == "INPUT" || el.tag_name() == "TEXTAREA"
                    });
                if key == "Escape" {
                    ui_open.set(false);
                    mobile_open.set(false);
                    customize_open.set(false);
                    return;
                }
                if key == "/" && !ev.meta_key() && !ev.ctrl_key() && !in_field {
                    ev.prevent_default();
                    if let Some(input) = search_ref.get() {
                        let _ = input.focus();
                    }
                    return;
                }
                if (key == "c" || key == "C")
                    && !ev.meta_key()
                    && !ev.ctrl_key()
                    && !in_field
                {
                    customize_open.update(|o| *o = !*o);
                }
            },
        ));
        let _ = window.add_event_listener_with_callback(
            "keydown",
            cb.as_ref().unchecked_ref(),
        );
        cb.forget();
    });

    // Segmented toggle order: Light · Device · Dark (device = system default).
    let theme_modes = [
        ("Light", ThemeMode::Light, Glyph::Sun),
        ("Device", ThemeMode::System, Glyph::Monitor),
        ("Dark", ThemeMode::Dark, Glyph::Moon),
    ];

    view! {
            <header class="sticky top-0 z-50 w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
                <div class="page-container flex h-16 items-center justify-between">
                    <div class="flex items-center gap-6">
    <a
                            href="/"
                            class="flex items-center gap-2 text-lg font-bold"
                            on:click={
                                let nav = navigate.clone();
                                move |ev| {
                                    ev.prevent_default();
                                    nav("/", Default::default());
                                }
                            }
                        >
                            <img src="/logo-64.png" alt="MontRS logo" class="h-7 w-7 rounded" />
                            "MontRS"
                        </a>
                        <nav class="hidden items-center gap-1 text-sm md:flex">
                            <a
                                href="/"
                                class="rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                on:click={
                                    let nav = navigate.clone();
                                    move |ev| {
                                        ev.prevent_default();
                                        nav("/", Default::default());
                                    }
                                }
                            >"Home"</a>
                            <div class="relative">
                                <button
                                    type="button"
                                    class="flex items-center gap-1 rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                    on:click=move |_| ui_open.update(|o| *o = !*o)
                                    aria-haspopup="menu"
                                    aria-expanded=move || ui_open.get()
                                >
                                    "UI"
                                    <Icon glyph=Glyph::ChevronDown class=move || {
                                        if ui_open.get() { "h-3 w-3 transition-transform rotate-180" } else { "h-3 w-3 transition-transform" }
                                    } />
                                </button>
                                <div
                                    class="fixed inset-0 z-40"
                                    hidden=move || !ui_open.get()
                                    on:click=move |_| ui_open.set(false)
                                ></div>
                                <div
                                    class="absolute left-0 z-50 mt-1 w-40 rounded-md border border-border bg-popover p-1 shadow-lg"
                                    hidden=move || !ui_open.get()
                                    role="menu"
                                    aria-label="UI"
                                >
                                    {ui_links.into_iter().map(|(href, label)| {
                                        let nav = navigate.clone();
                                        let close = ui_open;
                                        view! {
                                            <a
                                                href=href
                                                class="block rounded-sm px-3 py-1.5 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                                                on:click=move |ev| {
                                                    ev.prevent_default();
                                                    nav(href, Default::default());
                                                    close.set(false);
                                                }
                                            >{label}</a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                            {nav_links.into_iter().map(|(href, label)| {
                                let nav = navigate.clone();
                                view! {
                                    <a
                                        href=href
                                        class="rounded-md px-3 py-2 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            nav(href, Default::default());
                                        }
                                    >{label}</a>
                                }
                            }).collect::<Vec<_>>()}
                        </nav>
                    </div>

                    <div class="relative hidden md:block">
                        <Icon
                            glyph=Glyph::Search
                            class="pointer-events-none absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground"
                        />
                        <input
                            type="search"
                            placeholder="Search icons…  (/)"
                            class="h-8 w-40 rounded-md border border-input bg-background pl-8 pr-2 text-xs ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring lg:w-52"
                            node_ref=search_ref
                            prop:value=search_q
                            on:keydown=on_search_keydown
                            aria-label="Search icons"
                        />
                    </div>

                    <div class="relative flex items-center gap-2">
                        <GithubStars />

                        // Theme toggle: Light / System / Dark segmented pill
                        // (shark-ui style), defaults to System.
                        <div class="theme-toggle" role="group" aria-label="Theme">
                            <span
                                class="theme-toggle-indicator"
                                style=move || format!(
                                    "--theme-pos: {};",
                                    match theme.get() {
                                        ThemeMode::Light => 0,
                                        ThemeMode::System => 1,
                                        ThemeMode::Dark => 2,
                                    }
                                )
                            ></span>
                            {theme_modes.into_iter().map(|(label, mode, icon)| {
                                let mode2 = mode;
                                let is_active = move || theme.get() == mode2;
                                let select = move |_| theme.set(mode2);
                                view! {
                                    <button
                                        type="button"
                                        aria-label=label
                                        aria-pressed=is_active
                                        title=label
                                        on:click=select
                                    >
                                        <Icon glyph=icon class="h-4 w-4" />
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        // Mobile menu toggle
                        <button
                            type="button"
                            class="inline-flex h-9 w-9 items-center justify-center rounded-md border border-border text-muted-foreground transition-colors hover:bg-accent hover:text-foreground md:hidden"
                            on:click=move |_| mobile_open.update(|o| *o = !*o)
                            aria-label="Open menu"
                            aria-expanded=move || mobile_open.get()
                        >
                            <Icon glyph=Glyph::Menu class="h-4 w-4" />
                        </button>

                        <Show when=move || mobile_open.get()>
                            <div
                                class="fixed inset-0 z-40"
                                on:click=move |_| mobile_open.set(false)
                            ></div>
                            <div class="absolute right-0 top-12 z-50 w-52 rounded-md border border-border bg-popover p-1 shadow-lg md:hidden">
                                {core::iter::once(("/", "Home"))
                                    .chain(nav_links.iter().copied())
                                    .map(|(href, label)| {
                                    let nav = navigate.clone();
                                    let close_menu = mobile_open;
                                    view! {
                                        <a
                                            href=href
                                            class="block rounded-sm px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                nav(href, Default::default());
                                                close_menu.set(false);
                                            }
                                        >{label}</a>
                                    }
                                }).collect::<Vec<_>>()}
                                <p class="mt-1 border-t border-border px-3 pt-1.5 text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">
                                    "UI"
                                </p>
                                {ui_links.iter().copied().map(|(href, label)| {
                                    let nav = navigate.clone();
                                    let close_menu = mobile_open;
                                    view! {
                                        <a
                                            href=href
                                            class="block rounded-sm px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                nav(href, Default::default());
                                                close_menu.set(false);
                                            }
                                        >{label}</a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </Show>

                        // Customize button (shark-ui style): wand icon, opens a
                        // right sheet with gray / primary / radius controls.
                        <button
                            type="button"
                            class="inline-flex h-9 w-9 items-center justify-center rounded-md border border-border text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                            on:click=move |_| customize_open.update(|o| *o = !*o)
                            aria-label="Customize theme"
                            aria-expanded=move || customize_open.get()
                            title="Customize (C)"
                        >
                            <Icon glyph=Glyph::WandSparkles class="h-4 w-4" />
                        </button>

                        <Show when=move || customize_open.get()>
                            <div
                                class="fixed inset-0 z-40"
                                on:click=move |_| customize_open.set(false)
                            ></div>
                            <div class="fixed right-0 top-0 z-50 flex h-full w-full max-w-md flex-col border-l border-border bg-background shadow-xl">
                                <div class="flex items-start justify-between border-b border-border px-6 py-4">
                                    <div>
                                        <h2 class="text-lg font-semibold">"Make it yours"</h2>
                                        <p class="mt-0.5 text-sm text-muted-foreground">
                                            "Change the theme to match your style."
                                        </p>
                                    </div>
                                    <button
                                        type="button"
                                        class="inline-flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                        on:click=move |_| customize_open.set(false)
                                        aria-label="Close customize panel"
                                    >
                                        <Icon glyph=Glyph::X class="h-4 w-4" />
                                    </button>
                                </div>
                                <div class="flex-1 overflow-y-auto px-6 py-5">
                                    <ThemeCustomizer />
                                </div>
                            </div>
                        </Show>
                    </div>
                </div>
            </header>
        }
}

/// GitHub Star badge with a live star count fetched from the GitHub API
/// (client-side only; renders a plain "Star" link while SSR / fetching).
#[component]
fn GithubStars() -> impl IntoView {
    const REPO: &str = "https://github.com/afsall-inc/montrs";
    #[allow(dead_code)] // only referenced in the wasm32 fetch block
    const API: &str = "https://api.github.com/repos/afsall-inc/montrs";
    let stars = RwSignal::new(None::<u32>);

    #[cfg(target_arch = "wasm32")]
    {
        let fetched = RwSignal::new(false);
        let s = stars;
        Effect::new(move |_| {
            if fetched.get_untracked() {
                return;
            }
            fetched.set(true);
            // Show a cached count immediately so a rate-limited first load
            // still displays the last known value.
            if let Some(window) = web_sys::window()
                && let Ok(Some(raw)) = window.local_storage()
                && let Ok(Some(cached)) = raw.get_item("montrs-stars")
                && let Ok(n) = cached.parse::<u32>()
            {
                s.set(Some(n));
            }
            leptos::task::spawn_local(async move {
                use wasm_bindgen::JsCast;
                use wasm_bindgen_futures::JsFuture;
                let Some(window) = web_sys::window() else {
                    return;
                };
                let Ok(resp) = JsFuture::from(window.fetch_with_str(API)).await
                else {
                    return;
                };
                let Ok(resp) = resp.dyn_into::<web_sys::Response>() else {
                    return;
                };
                if !resp.ok() {
                    return; // rate-limited / transient — keep cached value
                }
                let Ok(text) = resp.text() else {
                    return;
                };
                let Ok(text) = JsFuture::from(text).await else {
                    return;
                };
                let Some(text) = text.as_string() else {
                    return;
                };
                let Ok(json) = serde_json::from_str::<serde_json::Value>(&text)
                else {
                    return;
                };
                let n = json
                    .get("stargazers_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                if n > 0 {
                    s.set(Some(n));
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ =
                            storage.set_item("montrs-stars", &n.to_string());
                    }
                }
            });
        });
    }

    let label = move || match stars.get() {
        Some(n) if n > 0 => format!("Star · {}", format_count(n)),
        _ => "Star".to_string(),
    };

    view! {
        <a
            href=REPO
            target="_blank"
            rel="noopener noreferrer"
            class="hidden items-center gap-1.5 rounded-md border border-border px-2.5 py-1.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground sm:inline-flex"
        >
            <Icon glyph=Glyph::Star class="h-3.5 w-3.5" />
            {label}
        </a>
    }
}

fn format_count(n: u32) -> String {
    if n >= 1_000 {
        let k = n as f64 / 1_000.0;
        if k >= 10.0 {
            format!("{:.0}k", k)
        } else {
            format!("{:.1}k", k)
        }
    } else {
        n.to_string()
    }
}

/// Minimal query-string encoding (spaces → `+`, others left as-is).
fn url_enc(s: &str) -> String {
    s.replace(' ', "+")
        .replace('#', "%23")
        .replace('&', "%26")
        .replace('=', "%3D")
}
