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
pub fn Faq01() -> impl IntoView {
    let open = RwSignal::new(Option::<usize>::None);
    let items = vec![
        (
            "What is MontRS?",
            "A full-stack Rust web framework for compile-time correctness and \
             agent-first development.",
        ),
        (
            "How do I install it?",
            "Run `cargo add montrs` or use `montrs new my-app` to scaffold a \
             new project.",
        ),
        (
            "Is it production ready?",
            "Yes — MontRS is used in production by early adopters. The API is \
             stabilizing.",
        ),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm divide-y divide-border">
            {items.into_iter().enumerate().map(|(i, (q, a))| {
                let is_open = move || open.get() == Some(i);
                let toggle = move |_| open.set(if is_open() { None } else { Some(i) });
                view! {
                    <div class="p-4">
                        <button on:click=toggle class="flex w-full items-center justify-between text-left">
                            <span class="text-sm font-medium">{q}</span>
                            <Icon glyph=Glyph::ChevronDown class=move || {
                                if is_open() { "w-4 h-4 text-muted-foreground rotate-180 transition-transform" }
                                else { "w-4 h-4 text-muted-foreground transition-transform" }
                            } />
                        </button>
                        <Show when=is_open>
                            <p class="mt-3 text-sm text-muted-foreground">{a}</p>
                        </Show>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
