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
use montrs_ui::prelude::*;

#[component]
pub fn Faq03() -> impl IntoView {
    let active = RwSignal::new("Getting Started");
    let sections = [
        (
            "Getting Started",
            &[
                (
                    "What is MontRS?",
                    "A full-stack Rust web framework for compile-time \
                     correctness.",
                ),
                (
                    "How do I install?",
                    "Run `cargo add montrs` or `montrs new my-app`.",
                ),
            ][..],
        ),
        (
            "Features",
            &[
                (
                    "Does it support WASM?",
                    "Yes — full WASM support for client-side rendering.",
                ),
                (
                    "Is there an ORM?",
                    "MontRS has a built-in ORM supporting PostgreSQL, SQLite, \
                     and MySQL.",
                ),
            ][..],
        ),
        (
            "Community",
            &[
                (
                    "How to contribute?",
                    "Check our contributing guide on GitHub.",
                ),
                ("Is there a Discord?", "Yes — join us at discord.gg/montrs."),
            ][..],
        ),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm p-6">
            <div class="flex flex-col md:flex-row gap-8">
                <nav class="md:w-48 shrink-0 space-y-1">
                    {sections.iter().map(|(title, _)| {
                        let t = *title;
                        let is_active = move || active.get() == t;
                        let click = move |_| active.set(t);
                        view! {
                            <button
                                on:click=click
                                class=move || {
                                    if is_active() {
                                        "w-full text-left px-3 py-2 rounded-md text-sm font-medium bg-primary/10 text-primary transition-colors"
                                    } else {
                                        "w-full text-left px-3 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
                                    }
                                }
                            >
                                {t}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </nav>
                <div class="flex-1 space-y-6">
                    {sections.iter().filter_map(|(title, items)| {
                        if active.get() != *title { return None; }
                        Some((title, items))
                    }).map(|(title, items)| {
                        view! {
                            <div>
                                <h3 class="text-lg font-semibold">{*title}</h3>
                                <div class="space-y-4">
                                    {items.iter().map(|(q, a)| {
                                        view! {
                                            <div>
                                                <h4 class="text-sm font-medium">{*q}</h4>
                                                <p class="mt-1 text-sm text-muted-foreground">{*a}</p>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
