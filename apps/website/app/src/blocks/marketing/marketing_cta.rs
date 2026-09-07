// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

/// Centered call-to-action banner with a gradient glow.
#[component]
pub fn MarketingCta() -> impl IntoView {
    view! {
        <section class="dot-grid hero-glow relative overflow-hidden rounded-2xl border border-border">
            <div class="px-6 py-16 text-center">
                <h4 class="text-2xl font-bold tracking-tight sm:text-3xl">
                    "Stop debugging nondeterminism."
                </h4>
                <p class="mx-auto mt-3 max-w-xl text-sm leading-6 text-muted-foreground">
                    "Describe it once. Run it everywhere — web, desktop, and mobile,
                    with the same AppSpec, the same tests, the same output."
                </p>
                <div class="mt-7 flex flex-wrap items-center justify-center gap-3">
                    <Button size=ButtonSize::Lg>"Get Started"</Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                        <Icon glyph=Glyph::Star class="mr-1.5 h-4 w-4" />
                        "Star on GitHub"
                    </Button>
                </div>
            </div>
        </section>
    }
}