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

//! Site header: sticky nav, ⌘K command palette, GitHub stars, theme cycle
//! button, and a mobile sheet. All navigation goes through [`NavLink`] so the
//! address bar updates immediately under the custom MontRS `RouterOutlet`.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use montrs_icons::*;
use montrs_ui::prelude::*;

use crate::components::NavLink;

/// `(label, href, icon)` — main navigation. The UI section (Components,
/// Blocks, Icons, Motion, Themes, Backgrounds) lives in its own sub-nav, so
/// the top bar only carries the framework-level destinations.
const NAV: &[(&str, &str, Glyph)] = &[
    ("UI", "/ui", Glyph::Component),
    ("Auth", "/auth", Glyph::ShieldCheck),
    ("Runtime", "/runtime", Glyph::Cpu),
    ("AI Kit", "/ai", Glyph::Bot),
    ("Foundations", "/foundations", Glyph::Blocks),
    ("Templates", "/templates", Glyph::LayoutTemplate),
    ("Packages", "/packages", Glyph::Package),
];

/// `(label, href, group)` — consumed by the command palette.
const COMMANDS: &[(&str, &str, &str)] = &[
    ("Home", "/", "Pages"),
    ("MontRS UI", "/ui", "Pages"),
    ("Components", "/ui/components", "Pages"),
    ("Blocks", "/ui/blocks", "Pages"),
    ("Icons", "/ui/icons", "Pages"),
    ("Motion", "/ui/motion", "Pages"),
    ("Themes", "/ui/themes", "Pages"),
    ("Backgrounds", "/ui/backgrounds", "Pages"),
    ("Packages", "/packages", "Pages"),
    ("Templates", "/templates", "Pages"),
    ("Auth", "/auth", "Framework"),
    ("Runtime", "/runtime", "Framework"),
    ("AI Kit", "/ai", "Framework"),
    ("Foundations", "/foundations", "Framework"),
    ("GitHub repository", "https://github.com/afsall-inc/montrs", "External"),
];

#[component]
pub fn Header() -> impl IntoView {
    let theme = use_theme();
    let navigate = use_navigate();
    let mobile_open = RwSignal::new(false);
    let palette_open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let selected = RwSignal::new(0usize);
    let input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    // Single cycle button: System → Light → Dark (system is the default).
    let theme_label = move || match theme.get() {
        ThemeMode::Light => "Light",
        ThemeMode::Dark => "Dark",
        ThemeMode::System => "System",
    };
    let theme_icon = move || match theme.get() {
        ThemeMode::Light => Glyph::Sun,
        ThemeMode::Dark => Glyph::Moon,
        ThemeMode::System => Glyph::Monitor,
    };
    let cycle_theme = move |_| {
        theme.update(|t| {
            *t = match t {
                ThemeMode::System => ThemeMode::Light,
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::System,
            }
        });
    };

    let filtered = Memo::new(move |_| {
        let q = query.get().trim().to_lowercase();
        COMMANDS
            .iter()
            .copied()
            .filter(|(label, _, _)| {
                q.is_empty() || label.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    });

    let activate = Callback::new({
        let navigate = navigate.clone();
        move |href: String| {
            palette_open.set(false);
            mobile_open.set(false);
            query.set(String::new());
            selected.set(0);
            if href.starts_with("http") {
                if let Some(window) = web_sys::window() {
                    let _ = window.open_with_url_and_target(&href, "_blank");
                }
            } else {
                navigate(&href, Default::default());
            }
        }
    });

    // Focus the palette input whenever it opens.
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if palette_open.get()
            && let Some(input) = input_ref.get()
        {
            let _ = input.focus();
        }
    });

    // Global shortcuts: ⌘K / Ctrl+K opens the palette, `/` opens it too.
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
                    palette_open.set(false);
                    mobile_open.set(false);
                    return;
                }
                if key == "k" && (ev.meta_key() || ev.ctrl_key()) {
                    ev.prevent_default();
                    palette_open.update(|o| *o = !*o);
                    query.set(String::new());
                    selected.set(0);
                    return;
                }
                if key == "/" && !ev.meta_key() && !ev.ctrl_key() && !in_field {
                    ev.prevent_default();
                    palette_open.set(true);
                    query.set(String::new());
                    selected.set(0);
                }
            },
        ));
        let _ = window.add_event_listener_with_callback(
            "keydown",
            cb.as_ref().unchecked_ref(),
        );
        cb.forget();
    });

    // Lock body scroll while the palette or mobile sheet is open.
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let locked = palette_open.get() || mobile_open.get();
        if let Some(document) = web_sys::window().and_then(|w| w.document())
            && let Some(body) = document.body()
        {
            let _ = body
                .style()
                .set_property("overflow", if locked { "hidden" } else { "" });
        }
    });

    view! {
        <header class="sticky top-0 z-50 w-full border-b border-border/60 bg-background/80 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <a
                href="#main-content"
                class="sr-only focus:not-sr-only focus:absolute focus:left-4 focus:top-3 focus:z-[80] focus:rounded-md focus:border focus:border-border focus:bg-background focus:px-3 focus:py-1.5 focus:text-sm"
            >"Skip to content"</a>
            <div class="page-container flex h-14 items-center gap-3">
                <NavLink
                    href="/"
                    class="flex shrink-0 items-center gap-2 text-base font-bold"
                >
                    <img src="/logo-64.png" alt="MontRS logo" class="h-6 w-6 rounded" />
                    "MontRS"
                </NavLink>

                <nav class="hidden items-center gap-0.5 text-sm lg:flex" aria-label="Main">
                    {NAV.iter().map(|(label, href, icon)| {
                        view! {
                            <NavLink
                                href=*href
                                class="inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                            >
                                <Icon glyph=*icon class="h-3.5 w-3.5" />
                                {*label}
                            </NavLink>
                        }
                    }).collect::<Vec<_>>()}
                </nav>

                <div class="ml-auto flex items-center gap-2">
                    <button
                        type="button"
                        class="hidden h-8 items-center gap-2 rounded-md border border-border bg-muted/40 px-2.5 text-xs text-muted-foreground transition-colors hover:bg-accent hover:text-foreground sm:inline-flex"
                        on:click=move |_| {
                            palette_open.set(true);
                            query.set(String::new());
                            selected.set(0);
                        }
                        aria-label="Search"
                    >
                        <Icon glyph=Glyph::Search class="h-3.5 w-3.5" />
                        "Search"
                        <kbd class="rounded border border-border bg-background px-1 font-mono text-[10px] text-muted-foreground">
                            "⌘K"
                        </kbd>
                    </button>

                    <GithubStars />

                    <button
                        type="button"
                        class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-border text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                        on:click=cycle_theme
                        aria-label=move || format!("Theme: {}", theme_label())
                        title=move || format!("Theme: {}", theme_label())
                    >
                        <Icon glyph=theme_icon class="h-4 w-4" />
                    </button>

                    <NavLink
                        href="/ui/components"
                        class="hidden h-8 items-center rounded-md bg-primary px-3 text-xs font-semibold text-primary-foreground transition-colors hover:bg-primary/90 md:inline-flex"
                    >"Get Started"</NavLink>

                    <button
                        type="button"
                        class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-border text-muted-foreground transition-colors hover:bg-accent hover:text-foreground lg:hidden"
                        on:click=move |_| mobile_open.update(|o| *o = !*o)
                        aria-label="Open menu"
                        aria-expanded=move || mobile_open.get()
                    >
                        <Icon glyph=Glyph::Menu class="h-4 w-4" />
                    </button>
                </div>
            </div>

            // Mobile sheet
            <Show when=move || mobile_open.get()>
                <div class="fixed inset-0 top-14 z-50 lg:hidden">
                    <div
                        class="fixed inset-0 bg-background/70 backdrop-blur-sm"
                        on:click=move |_| mobile_open.set(false)
                    ></div>
                    <nav
                        class="relative mx-4 mt-2 rounded-xl border border-border bg-popover p-2 shadow-xl"
                        aria-label="Mobile"
                    >
                        {NAV.iter().map(|(label, href, icon)| {
                            view! {
                                <a
                                    href=*href
                                    class="flex items-center gap-2 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                    on:click=move |ev| {
                                        ev.prevent_default();
                                        activate.run(href.to_string());
                                    }
                                >
                                    <Icon glyph=*icon class="h-4 w-4" />
                                    {*label}
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                        <div class="mt-1 flex items-center gap-2 border-t border-border px-2 pt-2">
                            <button
                                type="button"
                                class="inline-flex h-8 items-center gap-2 rounded-md border border-border px-2.5 text-xs text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                on:click=cycle_theme
                            >
                                <Icon glyph=theme_icon class="h-3.5 w-3.5" />
                                {theme_label}
                            </button>
                            <NavLink
                                href="/ui/components"
                                class="inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-semibold text-primary-foreground"
                            >"Get Started"</NavLink>
                        </div>
                    </nav>
                </div>
            </Show>

            // Command palette
            <Show when=move || palette_open.get()>
                <div
                    class="fixed inset-0 z-[70] flex items-start justify-center p-4 pt-[10vh]"
                    role="dialog"
                    aria-modal="true"
                    aria-label="Command palette"
                >
                    <div
                        class="fixed inset-0 bg-background/70 backdrop-blur-sm"
                        on:click=move |_| palette_open.set(false)
                    ></div>
                    <div class="relative w-full max-w-lg overflow-hidden rounded-xl border border-border bg-popover shadow-2xl">
                        <div class="flex items-center gap-2 border-b border-border px-3">
                            <Icon
                                glyph=Glyph::Search
                                class="h-4 w-4 shrink-0 text-muted-foreground"
                            />
                            <input
                                type="text"
                                class="h-11 w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
                                placeholder="Search pages and commands…"
                                node_ref=input_ref
                                prop:value=query
                                on:input=move |ev| {
                                    query.set(event_target_value(&ev));
                                    selected.set(0);
                                }
                                on:keydown={
                                    move |ev: leptos::ev::KeyboardEvent| {
                                        let items = filtered.get_untracked();
                                        match ev.key().as_str() {
                                            "ArrowDown" => {
                                                ev.prevent_default();
                                                if !items.is_empty() {
                                                    selected.update(|s| {
                                                        *s = (*s + 1).min(items.len() - 1);
                                                    });
                                                }
                                            }
                                            "ArrowUp" => {
                                                ev.prevent_default();
                                                selected.update(|s| {
                                                    *s = s.saturating_sub(1);
                                                });
                                            }
                                            "Enter" => {
                                                ev.prevent_default();
                                                if let Some((_, href, _)) =
                                                    items.get(selected.get_untracked())
                                                {
                                                    activate.run(href.to_string());
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            />
                            <kbd class="shrink-0 rounded border border-border bg-background px-1 font-mono text-[10px] text-muted-foreground">
                                "esc"
                            </kbd>
                        </div>
                        <ul class="max-h-80 overflow-y-auto p-1">
                            {move || {
                                filtered
                                    .get()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, (label, href, group))| {
                                        let is_selected = move || selected.get() == i;
                                        view! {
                                            <li>
                                                <button
                                                    type="button"
                                                    class=move || cn!(
                                                        "flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm transition-colors",
                                                        if is_selected() { "bg-accent text-accent-foreground" } else { "text-muted-foreground hover:bg-accent/60 hover:text-foreground" }
                                                    )
                                                    on:click=move |_| activate.run(href.to_string())
                                                >
                                                    <span>{label}</span>
                                                    <span class="text-[10px] uppercase tracking-wide text-muted-foreground">{group}</span>
                                                </button>
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                </div>
            </Show>
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
