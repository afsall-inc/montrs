// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

/// Split hero: headline, copy, and CTAs next to a code window.
#[component]
pub fn MarketingHero() -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 items-center gap-8 lg:grid-cols-2">
            <div>
                <span class="pill">
                    <span class="pill-accent">"New"</span>
                    "Shipped in v0.1.0"
                </span>
                <h3 class="mt-4 text-3xl font-bold tracking-tight sm:text-4xl">
                    "Ship your full-stack app in a day."
                </h3>
                <p class="mt-3 max-w-md text-sm leading-6 text-muted-foreground">
                    "One AppSpec, three targets — web, desktop, and mobile.
                    MontRS handles the rest, deterministically."
                </p>
                <div class="mt-6 flex flex-wrap gap-3">
                    <Button size=ButtonSize::Lg>"Get started"</Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                        "Read the docs"
                    </Button>
                </div>
                <div class="mt-6 flex flex-wrap items-center gap-4 text-xs text-muted-foreground">
                    {[
                        (Glyph::Check, "Deterministic"),
                        (Glyph::Check, "Agent-first"),
                        (Glyph::Check, "Cross-platform"),
                    ].into_iter().map(|(icon, label)| view! {
                        <span class="inline-flex items-center gap-1.5">
                            <Icon glyph=icon class="h-3.5 w-3.5 text-primary" />
                            {label}
                        </span>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="code-window">
                <div class="code-window-bar">
                    <span class="traffic-light traffic-light-red"></span>
                    <span class="traffic-light traffic-light-yellow"></span>
                    <span class="traffic-light traffic-light-green"></span>
                    <span class="code-window-tab">"montrs.toml"</span>
                </div>
                <pre class="code-window-body text-left">"# One spec, every target

[project]
name = \"my-app\"

[app]
target = \"web\"

[tasks]
ship = \"montrs build\"
"</pre>
            </div>
        </section>
    }
}
