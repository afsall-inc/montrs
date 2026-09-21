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

use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Docs() -> impl IntoView {
    let guides = [
        (
            "/router",
            Glyph::Route,
            "Router",
            "Typed routes, nested layouts, outlets, loaders, and guards.",
        ),
        (
            "/cli",
            Glyph::Terminal,
            "CLI",
            "Every command — scaffold, install, serve, build, and agent \
             tooling.",
        ),
        (
            "/runtime",
            Glyph::Cpu,
            "Runtime",
            "The native Rust runtime: ops, memory limits, permissions.",
        ),
        (
            "/foundations",
            Glyph::Blocks,
            "Foundations",
            "Plates, routes, signals, and the deterministic mental model.",
        ),
        (
            "/auth",
            Glyph::ShieldCheck,
            "Auth",
            "Email/password, OAuth, 2FA, sessions, and RBAC.",
        ),
        (
            "/ai",
            Glyph::Bot,
            "AI",
            "Agent sidecar, curated tools, and MCP integration.",
        ),
        (
            "/products",
            Glyph::Package,
            "Products",
            "Publishable packages for building full-stack apps.",
        ),
        (
            "/ui",
            Glyph::Component,
            "MontRS UI",
            "Components, blocks, icons, motion, themes, and backgrounds.",
        ),
    ];

    view! {
        <div class="page-container py-12">
            <div class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight">
                    "Docs"
                </h1>
                <p class="mt-2 max-w-2xl text-muted-foreground">
                    "Guides for the framework itself — routes, the CLI, the
                    runtime, and the packages behind them."
                </p>
            </div>
            <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                {guides.into_iter().map(|(href, icon, title, desc)| view! {
                    <a href=href class="showcase-card reveal flex flex-col p-6 transition-colors hover:border-ring/40">
                        <Icon glyph=icon class="h-6 w-6 text-primary" />
                        <h3 class="mt-4 font-semibold">{title}</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">{desc}</p>
                        <span class="mt-4 inline-flex items-center gap-1 text-sm font-medium text-primary">
                            "Open"
                            <Icon glyph=Glyph::ArrowRight class="h-4 w-4" />
                        </span>
                    </a>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
