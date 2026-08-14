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
pub fn Faq02() -> impl IntoView {
    let open = RwSignal::new(Option::<usize>::None);
    let items = vec![
        (
            "What is MontRS?",
            "A full-stack Rust web framework for compile-time correctness.",
        ),
        (
            "How do I get started?",
            "Run `montrs new my-app` and follow the golden path.",
        ),
        (
            "Is it production ready?",
            "Yes — actively used by early adopters.",
        ),
        (
            "Does it support WASM?",
            "Yes, MontRS compiles to WASM for full-stack apps.",
        ),
        (
            "What about databases?",
            "MontRS ORM supports PostgreSQL, SQLite, and MySQL.",
        ),
        (
            "Is there a community?",
            "Join our Discord and GitHub discussions.",
        ),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm p-6">
            <h3 class="text-lg font-semibold mb-6">"Frequently Asked Questions"</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                {items.into_iter().enumerate().map(|(i, (q, a))| {
                    let is_open = move || open.get() == Some(i);
                    let toggle = move |_| open.set(if is_open() { None } else { Some(i) });
                    view! {
                        <div class="flex gap-3">
                            <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-bold text-primary">
                                {i + 1}
                            </span>
                            <div>
                                <button on:click=toggle class="text-left">
                                    <h4 class="text-sm font-medium hover:text-primary transition-colors">{q}</h4>
                                </button>
                                <Show when=is_open>
                                    <p class="mt-1 text-xs text-muted-foreground">{a}</p>
                                </Show>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
