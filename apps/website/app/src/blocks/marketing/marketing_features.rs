// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

/// Three-column feature grid with icon headers.
#[component]
pub fn MarketingFeatures() -> impl IntoView {
    let items = [
        (
            Glyph::Zap,
            "Instant",
            "Hot reload, live-reload websockets, and zero-config dev.",
        ),
        (
            Glyph::Shield,
            "Safe",
            "Type-safe routes, compile-time validation, hermetic tests.",
        ),
        (
            Glyph::Bot,
            "Agent-ready",
            "Snapshots, skills, and MCP make your repo machine-readable.",
        ),
    ];
    view! {
        <section class="grid grid-cols-1 gap-4 md:grid-cols-3">
            {items.into_iter().map(|(icon, title, desc)| view! {
                <div class="showcase-card p-6">
                    <div class="inline-flex h-10 w-10 items-center justify-center rounded-md bg-primary/10 text-primary">
                        <Icon glyph=icon class="h-5 w-5" />
                    </div>
                    <h4 class="mt-4 font-semibold">{title}</h4>
                    <p class="mt-2 text-sm leading-6 text-muted-foreground">{desc}</p>
                </div>
            }).collect::<Vec<_>>()}
        </section>
    }
}