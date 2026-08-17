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
pub fn Integration07() -> impl IntoView {
    let hovered = RwSignal::new(Option::<&str>::None);
    let stats = vec![
        ("10K+", "GitHub Stars"),
        ("500+", "Contributors"),
        ("50K+", "Apps Built"),
        ("99.9%", "Uptime"),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                {stats.into_iter().map(|(value, label)| {
                    let l = label;
                    let is_hovered = move || hovered.get() == Some(l);
                    let mouseenter = move |_| hovered.set(Some(l));
                    let mouseleave = move |_| hovered.set(None);
                    view! {
                        <div
                            on:mouseenter=mouseenter
                            on:mouseleave=mouseleave
                            class=move || {
                                let base = "text-center rounded-lg p-4 transition-all cursor-default";
                                if is_hovered() {
                                    format!("{} bg-primary/5 scale-105", base)
                                } else {
                                    format!("{} hover:bg-muted/50", base)
                                }
                            }
                        >
                            <p class="text-3xl font-bold text-primary">{value}</p>
                            <p class="mt-1 text-sm text-muted-foreground">{label}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
