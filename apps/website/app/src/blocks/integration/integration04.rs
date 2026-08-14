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
pub fn Integration04() -> impl IntoView {
    let highlighted = RwSignal::new(Option::<String>::None);
    let features = vec![
        ("Compile-time safety", "Yes", "Yes", "No"),
        ("WASM support", "Yes", "Partial", "No"),
        ("Built-in ORM", "Yes", "No", "No"),
        ("Agent framework", "Yes", "No", "No"),
        ("Hot reload", "Yes", "Yes", "Yes"),
        ("TypeScript support", "N/A (Rust)", "Yes", "Yes"),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="border-b border-border bg-muted/50">
                            <th class="text-left px-4 py-3 font-medium">"Feature"</th>
                            <th class="text-left px-4 py-3 font-medium">"MontRS"</th>
                            <th class="text-left px-4 py-3 font-medium">"Framework A"</th>
                            <th class="text-left px-4 py-3 font-medium">"Framework B"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {features.into_iter().map(|(feature, a, b, c)| {
                            let f = feature;
                            let is_highlighted = move || highlighted.get().as_deref() == Some(f);
                            let click = move |_| highlighted.set(Some(f.to_string()));
                            view! {
                                <tr on:click=click class=move || {
                                    let base = "border-b border-border last:border-0 cursor-pointer transition-colors";
                                    if is_highlighted() {
                                        format!("{} bg-primary/5", base)
                                    } else {
                                        format!("{} hover:bg-muted/50", base)
                                    }
                                }>
                                    <td class="px-4 py-3 font-medium">{feature}</td>
                                    <td class="px-4 py-3">
                                        <span class="text-green-600 dark:text-green-400">{a}</span>
                                    </td>
                                    <td class="px-4 py-3 text-muted-foreground">{b}</td>
                                    <td class="px-4 py-3 text-muted-foreground">{c}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
