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
pub fn Home() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden">
            <div class="mx-auto max-w-6xl px-6 py-24 sm:py-32 lg:px-8">
                <div class="text-center">
                    <div class="flex justify-center mb-6">
                        <Icon glyph=Glyph::Blocks class="w-16 h-16 text-primary" size="64" />
                    </div>
                    <h1 class="text-4xl font-bold tracking-tight sm:text-6xl">
                        "MontRS"
                        <span class="text-primary block mt-2">"A full-stack Rust framework"</span>
                    </h1>
                    <p class="mt-6 text-lg leading-8 text-muted-foreground max-w-2xl mx-auto">
                        "Build web applications with compile-time correctness, explicit boundaries, and deterministic execution. Designed for humans and agents alike."
                    </p>
                    <div class="mt-10 flex items-center justify-center gap-4">
                        <a href="/components"
                            class="inline-flex items-center rounded-md bg-primary px-6 py-3 text-sm font-semibold text-primary-foreground shadow-sm hover:bg-primary/90 transition-colors"
                        >
                            "Browse Components"
                            <Icon glyph=Glyph::ArrowRight class="ml-2 w-4 h-4" />
                        </a>
                        <a href="/icons"
                            class="inline-flex items-center rounded-md border border-border px-6 py-3 text-sm font-semibold hover:bg-accent transition-colors"
                        >
                            "Browse Icons"
                            <Icon glyph=Glyph::Image class="ml-2 w-4 h-4" />
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <section class="border-t border-border py-20">
            <div class="mx-auto max-w-6xl px-6 lg:px-8">
                <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    <FeatureCard
                        icon=Glyph::Shield
                        title="Type-Safe"
                        description="Compile-time correctness with Rust's type system. No runtime surprises."
                    />
                    <FeatureCard
                        icon=Glyph::Puzzle
                        title="Modular Plates"
                        description="Compose your app from independent, reusable plates with clear boundaries."
                    />
                    <FeatureCard
                        icon=Glyph::Bot
                        title="Agent-First"
                        description="Machine-readable metadata, snapshots, and error tracking for AI coding partners."
                    />
                    <FeatureCard
                        icon=Glyph::Paintbrush
                        title="Tailwind CSS"
                        description="Beautiful UIs with Tailwind CSS and shadcn-inspired theming system."
                    />
                    <FeatureCard
                        icon=Glyph::Rocket
                        title="Fast Compilation"
                        description="Incremental compilation, WASM targets, and optimized build pipelines."
                    />
                    <FeatureCard
                        icon=Glyph::Heart
                        title="Open Source"
                        description="MIT licensed. Community-driven. Built for the future of web development."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    icon: Glyph,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-6 hover:shadow-md transition-shadow">
            <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
                <Icon glyph=icon class="w-6 h-6 text-primary" />
            </div>
            <h3 class="text-lg font-semibold text-card-foreground">{title}</h3>
            <p class="mt-2 text-sm text-muted-foreground">{description}</p>
        </div>
    }
}
