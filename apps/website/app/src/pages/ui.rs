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

use crate::copy::CopyButton;
use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

const USE_SNIPPET: &str = r#"use montrs_ui::prelude::*;

#[component]
fn Dashboard() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    view! {
        <Button>Save changes</Button>
        <Switch on=set_name />
        <Tabs items=["Overview", "Settings"] />
    }
}"#;

#[component]
pub fn Ui() -> impl IntoView {
    let stats = [
        ("91", "components"),
        ("22k+", "icons"),
        ("8", "collections"),
        ("6", "themes · backgrounds"),
    ];

    let sections = [
        (
            "/ui/components",
            Glyph::Blocks,
            "Components",
            "91 shadcn-inspired components built on Tailwind CSS. Buttons, \
             cards, inputs, tabs, toggles and more — copy the source, own \
             every pixel.",
        ),
        (
            "/ui/blocks",
            Glyph::LayoutTemplate,
            "Blocks",
            "Pre-built UI sections from real MontRS Plates: FAQs, footers, \
             headers, login screens, sidenavs, integrations.",
        ),
        (
            "/ui/icons",
            Glyph::Shapes,
            "Icons",
            "22,000+ icons across 8 license-safe collections: Lucide, Radix, \
             Tabler, Iconoir, Phosphor, MDI, Bootstrap, Simple Icons and \
             cryptocurrency logos — searchable, sizeable, animatable.",
        ),
        (
            "/ui/motion",
            Glyph::Activity,
            "Motion",
            "Spring physics, tween easings, keyframes, gesture helpers and \
             SVG path animation from the montrs-motion package.",
        ),
        (
            "/ui/themes",
            Glyph::Palette,
            "Themes",
            "Color-system presets, custom primaries and radius control — copy \
             one CSS block to theme an entire app.",
        ),
        (
            "/ui/backgrounds",
            Glyph::SwatchBook,
            "Backgrounds",
            "Curated page backdrops — dot grids, glows, grids and gradients — \
             tuned for both light and dark mode.",
        ),
    ];

    view! {
        <div class="page-container py-12">
            // -----------------------------------------------------------
            // Hero
            // -----------------------------------------------------------
            <div class="mx-auto max-w-3xl text-center">
                <div class="flex justify-center">
                    <span class="pill">
                        <span class="pill-accent">"MontRS UI"</span>
                        "Leptos · Tailwind · shadcn-inspired"
                    </span>
                </div>
                <h1 class="mt-6 text-4xl font-bold tracking-tight sm:text-6xl">
                    "Build interfaces at the speed of thought."
                </h1>
                <p class="mx-auto mt-6 max-w-2xl text-lg leading-8 text-muted-foreground">
                    "MontRS UI is the official component, block, icon, motion, and
                    theme library for MontRS apps. Beautifully designed, fully
                    reactive, and copy-anywhere — with zero runtime cost you
                    don't opt into."
                </p>
                <div class="mt-8 flex flex-wrap items-center justify-center gap-4">
                    <a
                        href="/ui/components"
                        class="inline-flex items-center rounded-md bg-primary px-6 py-3 text-sm font-semibold text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
                    >
                        "Browse components"
                        <Icon glyph=Glyph::ArrowRight class="ml-2 h-4 w-4" />
                    </a>
                    <a
                        href="/ui/icons"
                        class="inline-flex items-center rounded-md border border-border px-6 py-3 text-sm font-semibold transition-colors hover:bg-accent"
                    >
                        <Icon glyph=Glyph::Shapes class="mr-2 h-4 w-4" />
                        "Browse icons"
                    </a>
                </div>
                <div class="mt-10 grid grid-cols-2 gap-4 sm:grid-cols-4">
                    {stats.into_iter().map(|(n, label)| view! {
                        <div class="rounded-lg border border-border bg-card/60 px-4 py-3">
                            <p class="text-2xl font-bold">{n}</p>
                            <p class="mt-0.5 text-xs text-muted-foreground">{label}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // -----------------------------------------------------------
            // Install / add
            // -----------------------------------------------------------
            <div class="mx-auto mt-16 max-w-3xl">
                <h2 class="text-2xl font-semibold text-center">"Install into any MontRS app"</h2>
                <p class="mx-auto mt-2 max-w-xl text-center text-sm text-muted-foreground">
                    "Add the full library with cargo, or grab exactly what you need
                    with montrs add — shadcn-cli style, straight from the registry."
                </p>
                <div class="code-window mt-8">
                    <div class="code-window-bar">
                        <span class="traffic-light traffic-light-red"></span>
                        <span class="traffic-light traffic-light-yellow"></span>
                        <span class="traffic-light traffic-light-green"></span>
                        <span class="code-window-tab">"install"</span>
                    </div>
                    <div class="code-window-body">
                        <div class="flex items-center justify-between gap-3">
                            <span>
                                <span class="terminal-prompt">"$"</span>
                                " cargo add montrs-ui"
                            </span>
                            <CopyButton text="cargo add montrs-ui" label="Copy" />
                        </div>
                        <div class="mt-3 flex items-center justify-between gap-3">
                            <span>
                                <span class="terminal-prompt">"$"</span>
                                " montrs add button"
                            </span>
                            <CopyButton text="montrs add button" label="Copy" />
                        </div>
                        <div class="mt-3 flex items-center justify-between gap-3">
                            <span>
                                <span class="terminal-prompt">"$"</span>
                                " montrs add lucide/home --icon"
                            </span>
                            <CopyButton text="montrs add lucide/home --icon" label="Copy" />
                        </div>
                        <div class="mt-3 flex items-center justify-between gap-3">
                            <span>
                                <span class="terminal-prompt">"$"</span>
                                " montrs add tabler/arrow-right --icon"
                            </span>
                            <CopyButton text="montrs add tabler/arrow-right --icon" label="Copy" />
                        </div>
                        <div class="mt-3 flex items-center justify-between gap-3">
                            <span>
                                <span class="terminal-prompt">"$"</span>
                                " montrs add dark --theme"
                            </span>
                            <CopyButton text="montrs add dark --theme" label="Copy" />
                        </div>
                    </div>
                </div>
            </div>

            // -----------------------------------------------------------
            // Explore
            // -----------------------------------------------------------
            <div class="mt-16 grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                {sections.into_iter().map(|(href, icon, title, desc)| view! {
                    <a href=href class="showcase-card reveal flex flex-col p-6 transition-colors hover:border-ring/40">
                        <Icon glyph=icon class="h-6 w-6 text-primary" />
                        <h3 class="mt-3 font-semibold">{title}</h3>
                        <p class="mt-1.5 text-sm leading-6 text-muted-foreground">{desc}</p>
                        <span class="mt-4 inline-flex items-center gap-1 text-sm font-medium text-primary">
                            "Open"
                            <Icon glyph=Glyph::ArrowRight class="h-4 w-4" />
                        </span>
                    </a>
                }).collect::<Vec<_>>()}
            </div>

            // -----------------------------------------------------------
            // Use it
            // -----------------------------------------------------------
            <div class="mx-auto mt-16 max-w-3xl">
                <h2 class="text-2xl font-semibold text-center">"Use it anywhere"</h2>
                <p class="mx-auto mt-2 max-w-xl text-center text-sm text-muted-foreground">
                    "Every component is a Leptos component — reactive signals in,
                    DOM updates out. No providers required for the basics."
                </p>
                <div class="code-window mt-8">
                    <div class="code-window-bar">
                        <span class="traffic-light traffic-light-red"></span>
                        <span class="traffic-light traffic-light-yellow"></span>
                        <span class="traffic-light traffic-light-green"></span>
                        <span class="code-window-tab">"dashboard.rs"</span>
                    </div>
                    <div class="code-window-body">
                        <pre class="overflow-x-auto font-mono text-sm leading-6 text-foreground"><code>{USE_SNIPPET}</code></pre>
                    </div>
                </div>
            </div>
        </div>
    }
}
