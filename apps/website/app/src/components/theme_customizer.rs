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

//! Site-wide theme customization (shark-ui inspired). Presets are persisted to
//! localStorage and applied as CSS variables on `<html>` so every component
//! re-themes live. "Copy theme" emits a `:root` CSS snippet for your project.

use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};
use montrs_icons::*;
use montrs_ui::prelude::*;

/// (label, hsl triplet, foreground hsl)
const PRIMARY_OPTIONS: &[(&str, &str, &str)] = &[
    ("Rust", "24.6 94.8% 53.1%", "0 0% 100%"),
    ("Red", "0 84% 60%", "0 0% 100%"),
    ("Orange", "24 95% 53%", "0 0% 100%"),
    ("Amber", "38 92% 50%", "0 0% 100%"),
    ("Lime", "84 81% 44%", "120 100% 4%"),
    ("Green", "142 71% 45%", "0 0% 100%"),
    ("Teal", "172 66% 40%", "0 0% 100%"),
    ("Blue", "217 91% 60%", "0 0% 100%"),
    ("Violet", "258 90% 66%", "0 0% 100%"),
    ("Pink", "330 81% 60%", "0 0% 100%"),
];

/// (label, background hsl, foreground hsl, muted-foreground hsl, border hsl)
const GRAY_OPTIONS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "Near Black",
        "0 0% 4%",
        "0 0% 98%",
        "240 5% 65%",
        "0 0% 12%",
    ),
    (
        "Zinc",
        "240 10% 4%",
        "240 6% 98%",
        "240 5% 65%",
        "240 4% 16%",
    ),
    (
        "Slate",
        "240 6% 4%",
        "210 20% 98%",
        "215 16% 65%",
        "215 16% 14%",
    ),
    ("Stone", "24 10% 4%", "60 8% 98%", "24 6% 64%", "20 6% 14%"),
    ("Neutral", "0 0% 9%", "0 0% 98%", "0 0% 65%", "0 0% 20%"),
];

/// Light-mode neutral palettes (same families, `:root`-style). The theme
/// customizer applies the palette that matches the current Light/Device/Dark
/// mode so the mode toggle keeps controlling the whole site.
const LIGHT_GRAY_OPTIONS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "Near Black",
        "0 0% 100%",
        "0 0% 3.9%",
        "240 5% 46%",
        "240 6% 90%",
    ),
    (
        "Zinc",
        "240 5% 96%",
        "240 10% 3.9%",
        "240 5% 46%",
        "240 6% 90%",
    ),
    (
        "Slate",
        "210 20% 98%",
        "222 47% 11%",
        "215 16% 47%",
        "214 32% 91%",
    ),
    ("Stone", "60 9% 98%", "24 10% 10%", "24 6% 45%", "24 6% 83%"),
    ("Neutral", "0 0% 100%", "0 0% 3.9%", "0 0% 46%", "0 0% 89%"),
];

const RADIUS_OPTIONS: &[(&str, &str)] = &[
    ("Sharp", "0rem"),
    ("Small", "0.25rem"),
    ("Default", "0.5rem"),
    ("Large", "0.75rem"),
    ("Pill", "1rem"),
];

/// `(label, primary idx, gray idx, radius idx)` quick-pick combinations.
const PRESETS: &[(&str, usize, usize, usize)] = &[
    ("Rust", 0, 0, 2),
    ("Midnight", 7, 1, 2),
    ("Emerald", 5, 2, 1),
    ("Violet", 8, 1, 3),
    ("Stone", 3, 3, 2),
    ("Pill", 0, 4, 4),
];

#[allow(dead_code)]
const STORAGE_KEY: &str = "montrs-theme-config";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ThemeCfg {
    primary: usize, // index into PRIMARY_OPTIONS
    gray: usize,    // index into GRAY_OPTIONS
    radius: usize,  // index into RADIUS_OPTIONS
}

fn load_cfg() -> ThemeCfg {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window()
            && let Ok(Some(storage)) = window.local_storage()
            && let Ok(Some(raw)) = storage.get_item(STORAGE_KEY)
            && let Some((p, g, r)) =
                raw.split_once(',').and_then(|(a, rest)| {
                    rest.split_once(',').map(|(b, c)| (a, b, c))
                })
            && let (Ok(p), Ok(g), Ok(r)) =
                (p.parse::<usize>(), g.parse::<usize>(), r.parse::<usize>())
        {
            return ThemeCfg {
                primary: p.min(PRIMARY_OPTIONS.len() - 1),
                gray: g.min(GRAY_OPTIONS.len() - 1),
                radius: r.min(RADIUS_OPTIONS.len() - 1),
            };
        }
    }
    ThemeCfg {
        primary: 0,
        gray: 0,
        radius: 2,
    }
}

#[allow(unused_variables)]
fn save_cfg(cfg: ThemeCfg) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window()
            && let Ok(Some(storage)) = window.local_storage()
        {
            let _ = storage.set_item(
                STORAGE_KEY,
                &format!("{},{},{}", cfg.primary, cfg.gray, cfg.radius),
            );
        }
    }
}

#[allow(unused_variables)]
fn apply_cfg(cfg: ThemeCfg, dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        if let Some(document) = web_sys::window().and_then(|w| w.document())
            && let Some(doc_el) = document.document_element()
            && let Some(html) = doc_el.dyn_ref::<web_sys::HtmlElement>()
        {
            let s = html.style();
            for (key, value) in tokens_for(cfg, dark) {
                let _ = s.set_property(key, &value);
            }
        }
    }
}

/// Rotate the hue of an `H S% L%` triplet and return a new triplet.
fn rotate_hue(hsl: &str, delta: f64) -> String {
    let mut parts = hsl.split_whitespace();
    let hue = parts
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let sat = parts.next().unwrap_or("80%");
    let light = parts.next().unwrap_or("50%");
    let new_hue = (hue + delta).rem_euclid(360.0);
    format!("{new_hue:.1} {sat} {light}")
}

/// Full semantic token set for a configuration and color mode.
fn tokens_for(cfg: ThemeCfg, dark: bool) -> Vec<(&'static str, String)> {
    let (_, primary, primary_fg) = PRIMARY_OPTIONS[cfg.primary];
    let palette = if dark { GRAY_OPTIONS } else { LIGHT_GRAY_OPTIONS };
    let (_, bg, fg, muted_fg, border) = palette[cfg.gray];
    let (_, radius) = RADIUS_OPTIONS[cfg.radius];
    vec![
        ("--background", bg.to_string()),
        ("--foreground", fg.to_string()),
        ("--card", bg.to_string()),
        ("--card-foreground", fg.to_string()),
        ("--popover", bg.to_string()),
        ("--popover-foreground", fg.to_string()),
        ("--primary", primary.to_string()),
        ("--primary-foreground", primary_fg.to_string()),
        ("--secondary", border.to_string()),
        ("--secondary-foreground", fg.to_string()),
        ("--muted", border.to_string()),
        ("--muted-foreground", muted_fg.to_string()),
        ("--accent", border.to_string()),
        ("--accent-foreground", fg.to_string()),
        ("--border", border.to_string()),
        ("--input", border.to_string()),
        ("--ring", primary.to_string()),
        ("--radius", radius.to_string()),
        ("--chart-1", primary.to_string()),
        ("--chart-2", rotate_hue(primary, 55.0)),
        ("--chart-3", rotate_hue(primary, 130.0)),
        ("--chart-4", rotate_hue(primary, 200.0)),
        ("--chart-5", rotate_hue(primary, 280.0)),
    ]
}

fn copy_css(cfg: ThemeCfg) -> String {
    let mut out = String::from(":root {\n");
    for (key, value) in tokens_for(cfg, false) {
        out.push_str(&format!("  {key}: {value};\n"));
    }
    out.push_str("}\n\n.dark {\n");
    for (key, value) in tokens_for(cfg, true) {
        out.push_str(&format!("  {key}: {value};\n"));
    }
    out.push_str("}\n");
    out
}

#[component]
pub fn ThemeCustomizer() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // Read a shared preset from `?t=primary,gray,radius` so theme links work.
    let from_url = query.get().get("t").and_then(|raw| {
        let mut it = raw.split(',');
        let p = it.next()?.parse::<usize>().ok()?;
        let g = it.next()?.parse::<usize>().ok()?;
        let r = it.next()?.parse::<usize>().ok()?;
        Some(ThemeCfg {
            primary: p.min(PRIMARY_OPTIONS.len() - 1),
            gray: g.min(GRAY_OPTIONS.len() - 1),
            radius: r.min(RADIUS_OPTIONS.len() - 1),
        })
    });

    let cfg = RwSignal::new(from_url.unwrap_or_else(load_cfg));
    let copied = RwSignal::new(false);
    let theme = use_theme();

    let sync_url = {
        let navigate = navigate.clone();
        move |c: ThemeCfg| {
            let opts = leptos_router::NavigateOptions {
                replace: true,
                ..Default::default()
            };
            navigate(
                &format!("/ui/themes?t={},{},{}", c.primary, c.gray, c.radius),
                opts,
            );
        }
    };

    Effect::new(move |_| {
        let c = cfg.get();
        let dark = theme.get().is_dark();
        apply_cfg(c, dark);
        save_cfg(c);
        sync_url(c);
    });

    let flash_copied = move || {
        copied.set(true);
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let cb = wasm_bindgen::prelude::Closure::once_into_js(move || {
                copied.set(false)
            });
            if let Some(window) = web_sys::window() {
                let _ = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.unchecked_ref(),
                        1500,
                    );
            }
        }
    };

    let reset = move |_| {
        cfg.set(ThemeCfg {
            primary: 0,
            gray: 0,
            radius: 2,
        });
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(document) = web_sys::window().and_then(|w| w.document())
                && let Some(doc_el) = document.document_element()
                && let Some(html) = doc_el.dyn_ref::<web_sys::HtmlElement>()
            {
                let s = html.style();
                for (key, _) in tokens_for(
                    ThemeCfg {
                        primary: 0,
                        gray: 0,
                        radius: 2,
                    },
                    true,
                ) {
                    let _ = s.remove_property(key);
                }
            }
        }
    };

    view! {
        <div class="space-y-6">
            <div>
                <p class="mb-2 font-mono text-[11px] uppercase tracking-wide text-muted-foreground">
                    "Primary color"
                </p>
                <div class="grid grid-cols-5 gap-2">
                    {PRIMARY_OPTIONS.iter().enumerate().map(|(i, (label, hsl, _))| {
                        let i2 = i;
                        let is_active = move || cfg.get().primary == i2;
                        let swatch = format!("hsl({hsl})");
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    let base = "flex flex-col items-center gap-1 rounded-md border p-2 transition-colors";
                                    if is_active() {
                                        format!("{base} border-primary")
                                    } else {
                                        format!("{base} border-border hover:border-ring/50")
                                    }
                                }
                                on:click=move |_| cfg.update(|c| c.primary = i2)
                            >
                                <span class="h-5 w-5 rounded-full border border-border" style=format!("background-color: {swatch};")></span>
                                <span class="text-[10px] text-muted-foreground">{*label}</span>
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div>
                <p class="mb-2 font-mono text-[11px] uppercase tracking-wide text-muted-foreground">
                    "Background"
                </p>
                <div class="grid grid-cols-5 gap-2">
                    {(0..GRAY_OPTIONS.len()).map(|i| {
                        let i2 = i;
                        let is_active = move || cfg.get().gray == i2;
                        let label = GRAY_OPTIONS[i2].0.to_string();
                        let swatch = move || {
                            let palette = if theme.get().is_dark() {
                                GRAY_OPTIONS
                            } else {
                                LIGHT_GRAY_OPTIONS
                            };
                            let (_, bg, fg, _, _) = palette[i2];
                            format!("background-color: hsl({bg}); color: hsl({fg});")
                        };
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    let base = "flex flex-col items-center gap-1 rounded-md border p-2 transition-colors";
                                    if is_active() {
                                        format!("{base} border-primary")
                                    } else {
                                        format!("{base} border-border hover:border-ring/50")
                                    }
                                }
                                on:click=move |_| cfg.update(|c| c.gray = i2)
                            >
                                <span class="h-5 w-5 rounded-full border border-border" style=swatch></span>
                                <span class="text-[10px] text-muted-foreground">{label}</span>
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div>
                <p class="mb-2 font-mono text-[11px] uppercase tracking-wide text-muted-foreground">
                    "Border radius"
                </p>
                <div class="flex flex-wrap gap-2">
                    {RADIUS_OPTIONS.iter().enumerate().map(|(i, (label, _))| {
                        let i2 = i;
                        let is_active = move || cfg.get().radius == i2;
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    let base = "rounded-full border px-3 py-1 text-xs font-medium transition-colors";
                                    if is_active() {
                                        format!("{base} border-primary bg-primary/10 text-primary")
                                    } else {
                                        format!("{base} border-border text-muted-foreground hover:bg-accent hover:text-foreground")
                                    }
                                }
                                on:click=move |_| cfg.update(|c| c.radius = i2)
                            >{*label}</button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div>
                <p class="mb-2 font-mono text-[11px] uppercase tracking-wide text-muted-foreground">
                    "Quick presets"
                </p>
                <div class="flex flex-wrap gap-2">
                    {PRESETS.iter().map(|(label, p, g, r)| {
                        let (p, g, r) = (*p, *g, *r);
                        view! {
                            <button
                                type="button"
                                class="rounded-full border border-border px-3 py-1 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                on:click=move |_| cfg.set(ThemeCfg { primary: p, gray: g, radius: r })
                            >{*label}</button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div class="flex flex-wrap items-center gap-2 border-t border-border pt-4">
                <button
                    type="button"
                    class=move || {
                        let base = "copy-btn inline-flex items-center gap-1";
                        if copied.get() {
                            format!("{base} border-transparent bg-primary/15 text-primary")
                        } else {
                            base.to_string()
                        }
                    }
                    on:click=move |_| {
                        crate::copy::copy_text(&copy_css(cfg.get()));
                        flash_copied();
                    }
                >
                    {move || if copied.get() { "Copied".to_string() } else { "Copy theme CSS".to_string() }}
                </button>
                <button
                    type="button"
                    class="inline-flex items-center gap-1.5 rounded-md border border-border px-3 py-1.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                    on:click=move |_| {
                        #[cfg(target_arch = "wasm32")]
                        let url = {
                            let c = cfg.get();
                            web_sys::window()
                                .map(|w| {
                                    let loc = w.location();
                                    format!(
                                        "{}{}?t={},{},{}",
                                        loc.origin().unwrap_or_default(),
                                        loc.pathname().unwrap_or_default(),
                                        c.primary,
                                        c.gray,
                                        c.radius
                                    )
                                })
                                .unwrap_or_default()
                        };
                        #[cfg(not(target_arch = "wasm32"))]
                        let url = String::new();
                        crate::copy::copy_text(&url);
                        flash_copied();
                    }
                >
                    <Icon glyph=Glyph::Link class="h-3.5 w-3.5" />
                    "Copy link"
                </button>
                <button
                    type="button"
                    class="inline-flex items-center gap-1.5 rounded-md border border-border px-3 py-1.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                    on:click=reset
                >
                    <Icon glyph=Glyph::RotateCcw class="h-3.5 w-3.5" />
                    "Reset"
                </button>
            </div>
        </div>
    }
}
