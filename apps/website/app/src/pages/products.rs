// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Products and publishable ecosystem packages showcase.
//!
//! Spotlights publishable crates and modules designed for developers
//! to build applications with MontRS (Auth, UI, Icons, Motion, State, ORM, etc.).

use crate::copy::CopyButton;
use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

struct Product {
    name: &'static str,
    crate_name: &'static str,
    tagline: &'static str,
    description: &'static str,
    category: &'static str,
    features: &'static [&'static str],
    href: Option<&'static str>,
    glyph: Glyph,
}

const PRODUCTS: &[Product] = &[
    Product {
        name: "MontRS UI",
        crate_name: "montrs-ui",
        tagline: "Accessible, shadcn-inspired components for Leptos",
        description: "A comprehensive library of 90+ accessible components \
                      with built-in Tailwind styling, theme customizer, toast \
                      system, and zero runtime JavaScript.",
        category: "UI & Design",
        features: &[
            "Accessible keyboard navigation",
            "Automatic dark & light themes",
            "90+ components & blocks",
            "Tailwind CSS v4 native",
        ],
        href: Some("/ui"),
        glyph: Glyph::Component,
    },
    Product {
        name: "MontRS Icons",
        crate_name: "montrs-icons",
        tagline: "22,000+ icons across 9 popular icon families",
        description: "Lucide, Radix, Tabler, Phosphor, Iconoir, Material \
                      Design, Bootstrap, Simple Icons, and Crypto icons \
                      compiled directly into zero-cost Leptos SVG components.",
        category: "UI & Design",
        features: &[
            "Lucide, Radix, Phosphor & Tabler",
            "Tree-shakeable per-icon imports",
            "Dynamic Glyph enum lookup",
            "Animated icon variants",
        ],
        href: Some("/ui/icons"),
        glyph: Glyph::Smile,
    },
    Product {
        name: "MontRS Motion",
        crate_name: "montrs-motion",
        tagline: "Physics-based springs, tweens, and gesture animations",
        description: "Fluid springs, SVG morphing, CSS transitions, and \
                      gesture recognition built with WebAssembly performance \
                      and zero jank.",
        category: "UI & Design",
        features: &[
            "Spring physics engine",
            "Keyframe interpolators",
            "Gesture & drag hooks",
            "Hardware-accelerated CSS/SVG",
        ],
        href: Some("/ui/motion"),
        glyph: Glyph::Sparkles,
    },
    Product {
        name: "MontRS Auth",
        crate_name: "montrs-auth",
        tagline: "Full-stack authentication, sessions, and RBAC",
        description: "Complete authentication system with email/password, \
                      OAuth providers (GitHub, Google, Apple), 2FA/TOTP, \
                      encrypted cookie sessions, and role-based access \
                      control.",
        category: "Full-stack",
        features: &[
            "OAuth2 with popular providers",
            "2FA & TOTP authenticator app support",
            "Secure encrypted session cookies",
            "Granular RBAC role policies",
        ],
        href: Some("/auth"),
        glyph: Glyph::ShieldCheck,
    },
    Product {
        name: "MontRS AI Kit",
        crate_name: "montrs-agent",
        tagline: "AI Agent sidecar, MCP tools, and machine-readable specs",
        description: "Embedded AI developer tools with Model Context Protocol \
                      (MCP) server support, automated code doctor, error \
                      tracking, and self-documenting project snapshots.",
        category: "Developer Tools",
        features: &[
            "MCP server protocol out-of-the-box",
            "Live snapshot generation (`agent.json`)",
            "Automated error triage & diffs",
            "Agent-callable `@agent-tool` annotations",
        ],
        href: Some("/ai"),
        glyph: Glyph::Bot,
    },
    Product {
        name: "MontRS ORM",
        crate_name: "montrs-orm",
        tagline: "SQL-first, type-safe database persistence",
        description: "Async-first SQL database mapping with support for \
                      PostgreSQL, SQLite, and MySQL. Compile-time checked \
                      queries and lightweight model derives.",
        category: "Full-stack",
        features: &[
            "Async connection pooling",
            "Compile-time query verification",
            "SQLite, PostgreSQL & MySQL drivers",
            "Seamless Loader/Action integration",
        ],
        href: None,
        glyph: Glyph::Database,
    },
    Product {
        name: "MontRS State",
        crate_name: "montrs-state",
        tagline: "Deterministic stores, state machines, and undo history",
        description: "Serializable state management designed for \
                      predictability, hot reload preservation, time-travel \
                      debugging, and multi-window state synchronization.",
        category: "State Management",
        features: &[
            "Deterministic transitions",
            "Built-in undo/redo history",
            "Hot-reload state preservation",
            "Cross-target synchronization",
        ],
        href: None,
        glyph: Glyph::Workflow,
    },
    Product {
        name: "MontRS Hotkeys",
        crate_name: "montrs-hotkeys-web",
        tagline: "Cross-platform keyboard shortcut engine",
        description: "Declarative shortcut parsing and matching for web, \
                      desktop, and embedded environments with modifier key \
                      normalization and conflict resolution.",
        category: "UI & Design",
        features: &[
            "Cmd/Ctrl cross-platform normalization",
            "Chord and key sequence support",
            "Input field focus protection",
            "Command palette integration",
        ],
        href: None,
        glyph: Glyph::Keyboard,
    },
    Product {
        name: "MontRS Haptics",
        crate_name: "montrs-haptics",
        tagline: "Tactile haptic feedback for web and mobile",
        description: "Unified haptic API providing tactile responses for \
                      clicks, impacts, selection changes, and gestures across \
                      mobile browsers and native apps.",
        category: "UI & Design",
        features: &[
            "Light, medium, heavy impact styles",
            "Selection click pulses",
            "Graceful fallback when unsupported",
            "Native mobile bridge ready",
        ],
        href: None,
        glyph: Glyph::Smartphone,
    },
    Product {
        name: "MontRS Content",
        crate_name: "montrs-content",
        tagline: "Typed Markdown & MDX content collections",
        description: "Static and dynamic content collections with YAML/TOML \
                      frontmatter parsing, syntax highlighting, deterministic \
                      table of contents, and asset co-location.",
        category: "Full-stack",
        features: &[
            "Schema-validated frontmatter",
            "Built-in code syntax highlighting",
            "Deterministic sorting and filtering",
            "Auto-generated heading anchors",
        ],
        href: None,
        glyph: Glyph::FileText,
    },
];

#[component]
pub fn Products() -> impl IntoView {
    let filter = RwSignal::new("All".to_string());
    let categories = [
        "All",
        "UI & Design",
        "Full-stack",
        "Developer Tools",
        "State Management",
    ];

    let filtered = move || {
        let f = filter.get();
        PRODUCTS
            .iter()
            .filter(|p| f == "All" || p.category == f)
            .collect::<Vec<_>>()
    };

    view! {
        <div class="page-container py-16 sm:py-24">
            // Hero
            <div class="mx-auto max-w-3xl text-center">
                <div class="inline-flex items-center gap-2 rounded-full border border-primary/20 bg-primary/10 px-3.5 py-1 text-xs font-semibold text-primary">
                    <Icon glyph=Glyph::Boxes class="h-3.5 w-3.5" />
                    "Ecosystem & Products"
                </div>
                <h1 class="mt-5 text-4xl font-bold tracking-tight sm:text-5xl">
                    "Everything you need to build production apps."
                </h1>
                <p class="mt-4 text-lg text-muted-foreground">
                    "Modular, publishable crates engineered specifically for MontRS developers. Install only what you need, combine seamlessly."
                </p>
            </div>

            // Category Filter Pills
            <div class="mt-12 flex flex-wrap items-center justify-center gap-2">
                {categories.into_iter().map(|cat| {
                    let cat_val = cat.to_string();
                    let cat_click = cat_val.clone();
                    let active = move || filter.get() == cat_val;
                    view! {
                        <button
                            type="button"
                            class=move || {
                                if active() {
                                    "rounded-full border border-primary bg-primary px-4 py-1.5 text-xs font-semibold text-primary-foreground shadow-sm transition-all"
                                } else {
                                    "rounded-full border border-border bg-card px-4 py-1.5 text-xs font-medium text-muted-foreground transition-all hover:bg-accent hover:text-foreground"
                                }
                            }
                            on:click=move |_| filter.set(cat_click.clone())
                        >
                            {cat}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Products Grid
            <div class="mt-12 grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                <For
                    each=move || filtered()
                    key=|p| p.crate_name
                    children=move |product| {
                        let crate_name = product.crate_name;
                        let cargo_cmd = format!("cargo add {}", crate_name);
                        view! {
                            <div class="group relative flex flex-col justify-between rounded-2xl border border-border bg-card/60 p-6 backdrop-blur transition-all hover:border-primary/50 hover:shadow-lg">
                                <div>
                                    <div class="flex items-center justify-between gap-3">
                                        <div class="flex h-11 w-11 items-center justify-center rounded-xl bg-primary/10 text-primary transition-colors group-hover:bg-primary group-hover:text-primary-foreground">
                                            <Icon glyph=product.glyph class="h-5 w-5" />
                                        </div>
                                        <span class="rounded-full border border-border/80 bg-muted/40 px-2.5 py-0.5 text-[11px] font-medium text-muted-foreground">
                                            {product.category}
                                        </span>
                                    </div>

                                    <h3 class="mt-4 text-xl font-bold tracking-tight text-foreground">
                                        {product.name}
                                    </h3>
                                    <p class="mt-1 text-xs font-mono text-primary font-semibold">
                                        {product.crate_name}
                                    </p>
                                    <p class="mt-2.5 text-sm font-medium text-foreground/90">
                                        {product.tagline}
                                    </p>
                                    <p class="mt-2 text-xs leading-relaxed text-muted-foreground">
                                        {product.description}
                                    </p>

                                    <ul class="mt-4 space-y-1.5 border-t border-border/60 pt-4">
                                        {product.features.iter().map(|f| {
                                            view! {
                                                <li class="flex items-center gap-2 text-xs text-muted-foreground">
                                                    <Icon glyph=Glyph::Check class="h-3.5 w-3.5 shrink-0 text-primary" />
                                                    <span>{*f}</span>
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </div>

                                <div class="mt-6 flex items-center justify-between gap-2 border-t border-border/60 pt-4">
                                    <div class="flex items-center gap-1.5 rounded-lg border border-border bg-muted/30 px-2.5 py-1 text-[11px] font-mono text-muted-foreground">
                                        <span>"$"</span>
                                        <span class="truncate max-w-[140px]">{cargo_cmd.clone()}</span>
                                        <CopyButton text=cargo_cmd label="" />
                                    </div>

                                    {if let Some(href) = product.href {
                                        view! {
                                            <a
                                                href=href
                                                class="inline-flex items-center gap-1 rounded-md px-2.5 py-1 text-xs font-semibold text-primary transition-colors hover:bg-primary/10"
                                            >
                                                "Explore"
                                                <Icon glyph=Glyph::ArrowRight class="h-3 w-3" />
                                            </a>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <span class="text-[11px] font-medium text-muted-foreground/70">
                                                "Built-in"
                                            </span>
                                        }.into_any()
                                    }}
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            // Quickstart Callout
            <div class="mt-16 rounded-2xl border border-border bg-gradient-to-r from-primary/10 via-background to-primary/5 p-8 text-center sm:p-12">
                <h2 class="text-2xl font-bold tracking-tight sm:text-3xl">
                    "Start building in seconds"
                </h2>
                <p class="mx-auto mt-2 max-w-xl text-sm text-muted-foreground">
                    "Scaffold a brand new project with auth, full-stack routing, and the UI component library pre-configured."
                </p>
                <div class="mt-6 flex flex-wrap items-center justify-center gap-3">
                    <div class="inline-flex items-center gap-3 rounded-xl border border-border bg-background px-4 py-2 font-mono text-sm shadow-inner">
                        <span class="terminal-prompt">"$"</span>
                        <span>"montrs new my-app --template saas"</span>
                        <CopyButton text="montrs new my-app --template saas" label="Copy" />
                    </div>
                </div>
            </div>
        </div>
    }
}
