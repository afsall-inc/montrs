// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The MontRS testing story: test every layer in-process.

use crate::{
    copy::CopyButton, highlight::highlight_rust, pages::home::CodeWindow,
};
use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

const LAYERS: &[(&str, &str, &str, Glyph)] = &[
    (
        "Frontend",
        "Render any view! in-process and query the DOM by selector — text, \
         classes, attributes, and ARIA roles.",
        "ComponentTest::render",
        Glyph::Component,
    ),
    (
        "Layout & overflow",
        "Build a real box model and assert nothing overflows horizontally at \
         a given viewport width. No browser required.",
        "SimLayout::compute",
        Glyph::Ruler,
    ),
    (
        "Backend & APIs",
        "Drive the app's router without a socket: requests, status codes, \
         JSON bodies, loaders, and actions.",
        "TestClient::get",
        Glyph::Server,
    ),
    (
        "Databases",
        "Real SQL in memory, or a recording backend that captures every \
         statement your code runs.",
        "RecordingDb::ran",
        Glyph::Database,
    ),
    (
        "Animations",
        "Step springs, tweens, and keyframes over a virtual timeline and \
         assert exact values and settle times.",
        "MotionTest::step_ms",
        Glyph::Sparkles,
    ),
    (
        "Time & randomness",
        "An injectable clock and a seeded RNG make retries, rate limits, and \
         flaky paths fully deterministic.",
        "TestClock::advance",
        Glyph::Clock,
    ),
];

const API_SNIPPET: &str = r#"use montrs_test::prelude::*;

#[tokio::test]
async fn api_and_loader() -> anyhow::Result<()> {
    let harness = TestHarness::new(build_spec()).seed(42);

    // No server: the router renders the request in-process.
    let client = TestClient::new(&harness.spec, || view! { <App /> })?;
    client.get("/health").assert_status(200);

    // Run a loader directly and inspect its JSON output.
    let user = harness.load("/users/:id").await?;
    assert_eq!(user["name"], "Ada");
    Ok(())
}"#;

const UI_SNIPPET: &str = r#"use montrs_test::prelude::*;

#[test]
fn card_is_accessible_and_fits() {
    let view = ComponentTest::render(|| view! { <Card title="Hi" /> });

    view.assert_role("button", "Save");
    view.assert_text("Hi");

    // Hermetic overflow evaluation for every target viewport.
    for width in [320.0, 768.0, 1280.0] {
        SimLayout::compute(view.html(), Viewport::width(width))
            .assert_no_horizontal_overflow();
    }
}"#;

const MOTION_SNIPPET: &str = r#"use montrs_test::prelude::*;

#[test]
fn spring_settles_quickly() {
    let spring = Spring::new(100.0, 10.0, 1.0).with_range(0.0, 1.0);
    let mut motion = MotionTest::new(spring);

    let frame = motion.step_ms(16);
    assert!(frame > 0.0 && frame < 1.0);

    motion.assert_settles_within_ms(600, 0.01);
}"#;

const REGRESSION_SNIPPET: &str = r#"# Run regression checks against committed baselines
montrs verify

# Check only responsive layout and overflow
montrs verify --ui

# Check only API contracts and route schemas
montrs verify --api

# Run determinism self-consistency (execute twice, assert byte-identical)
montrs verify --self-check

# Update baselines with newly validated values
montrs verify --update"#;

const CLI_SNIPPET: &str = "cargo add montrs-test --features \
                           http,db,sim-dom,layout,motion,mock,traffic,fuzz,\
                           macros";

#[component]
pub fn Testing() -> impl IntoView {
    view! {
        <div class="page-container py-16 sm:py-24">
            <div class="mx-auto max-w-3xl text-center">
                <div class="inline-flex items-center gap-2 rounded-full border border-primary/20 bg-primary/10 px-3.5 py-1 text-xs font-semibold text-primary">
                    <Icon glyph=Glyph::FlaskConical class="h-3.5 w-3.5" />
                    "Deterministic Test Fabric"
                </div>
                <h1 class="mt-5 text-4xl font-bold tracking-tight sm:text-5xl">
                    "Test everything without running anything."
                </h1>
                <p class="mt-4 text-lg text-muted-foreground">
                    "The frontend, backend, APIs, databases, UI, motion, and all three
                    deployment targets are testable in-process — no server, no browser,
                    no external service. Every result is deterministic and hermetic."
                </p>
                <div class="mt-8 flex flex-wrap items-center justify-center gap-3">
                    <div class="inline-flex items-center gap-3 rounded-xl border border-border bg-card px-4 py-2 font-mono text-sm shadow-inner">
                        <span class="terminal-prompt">"$"</span>
                        <span>"cargo add montrs-test"</span>
                        <CopyButton text=CLI_SNIPPET label="Copy" />
                    </div>
                    <a
                        href="/docs"
                        class="inline-flex items-center rounded-md border border-border px-4 py-2 text-sm font-semibold transition-colors hover:bg-accent"
                    >
                        "Read the testing guide"
                    </a>
                </div>
            </div>

            // Layer grid
            <div class="mt-16 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {LAYERS.iter().map(|(title, body, api, glyph)| {
                    view! {
                        <div class="showcase-card flex flex-col p-6">
                            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 text-primary">
                                <Icon glyph=*glyph class="h-5 w-5" />
                            </div>
                            <h3 class="mt-4 font-semibold">{*title}</h3>
                            <p class="mt-2 flex-1 text-sm text-muted-foreground">{*body}</p>
                            <code class="mt-4 rounded-md border border-border bg-muted/40 px-2 py-1 font-mono text-xs text-primary">
                                {*api}
                            </code>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Code samples
            <div class="mt-20 grid grid-cols-1 gap-8 lg:grid-cols-2">
                <section>
                    <h2 class="text-2xl font-bold tracking-tight">
                        "Backend & APIs"
                    </h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Render requests and run loaders directly through the router."
                    </p>
                    <div class="mt-5">
                        <CodeWindow tab="api_test.rs" body=move || highlight_rust(API_SNIPPET) />
                    </div>
                </section>
                <section>
                    <h2 class="text-2xl font-bold tracking-tight">
                        "Components & overflow"
                    </h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Query the DOM by selector and assert layout fits every viewport."
                    </p>
                    <div class="mt-5">
                        <CodeWindow tab="ui_test.rs" body=move || highlight_rust(UI_SNIPPET) />
                    </div>
                </section>
                <section>
                    <h2 class="text-2xl font-bold tracking-tight">
                        "Animation timelines"
                    </h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Advance a virtual clock frame by frame and assert exact values."
                    </p>
                    <div class="mt-5">
                        <CodeWindow tab="motion_test.rs" body=move || highlight_rust(MOTION_SNIPPET) />
                    </div>
                </section>
                <section>
                    <h2 class="text-2xl font-bold tracking-tight">
                        "Ready-made mocks"
                    </h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Swap real infrastructure for deterministic doubles."
                    </p>
                    <ul class="mt-5 space-y-3 text-sm">
                        <li class="flex items-start gap-3">
                            <Icon glyph=Glyph::Database class="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                            <span>
                                <strong class="text-foreground">"sqlite_memory / RecordingDb / MockDb"</strong>
                                <span class="text-muted-foreground">" — real SQL in memory, statement recording, and execute expectations."</span>
                            </span>
                        </li>
                        <li class="flex items-start gap-3">
                            <Icon glyph=Glyph::Clock class="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                            <span>
                                <strong class="text-foreground">"TestClock / TestRng"</strong>
                                <span class="text-muted-foreground">" — time that only moves when you advance it, and randomness you can replay."</span>
                            </span>
                        </li>
                        <li class="flex items-start gap-3">
                            <Icon glyph=Glyph::Layers class="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                            <span>
                                <strong class="text-foreground">"One harness, many targets"</strong>
                                <span class="text-muted-foreground">" — the same app spec drives web, desktop, and mobile tests."</span>
                            </span>
                        </li>
                    </ul>
                </section>
                <section class="lg:col-span-2">
                    <div class="rounded-2xl border border-border/80 bg-card/60 p-6 backdrop-blur sm:p-8">
                        <div class="inline-flex items-center gap-2 rounded-full border border-primary/20 bg-primary/10 px-3 py-1 text-xs font-semibold text-primary">
                            <Icon glyph=Glyph::CheckCheck class="h-3.5 w-3.5" />
                            "Deterministic Regression CI"
                        </div>
                        <h3 class="mt-3 text-2xl font-bold tracking-tight">
                            "montrs verify — Gate regressions before they land"
                        </h3>
                        <p class="mt-3 text-sm leading-relaxed text-muted-foreground">
                            "Available both in the MontRS framework itself and for any application built with MontRS. Developers and CI pipelines run <code>montrs verify</code> to evaluate performance budgets, responsive constraints, and API schemas against committed <code>.montrs/baselines/</code>."
                        </p>
                        <div class="mt-6">
                            <pre class="overflow-x-auto rounded-lg border border-border bg-muted/30 p-4 font-mono text-xs text-foreground">
                                {REGRESSION_SNIPPET}
                            </pre>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}
