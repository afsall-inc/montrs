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
pub fn Sidenav08() -> impl IntoView {
    let open = RwSignal::new(false);
    let active = RwSignal::new("Dashboard");

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <button on:click=move |_| active.set("Dashboard") class=move || {
                    let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                    if active.get() == "Dashboard" {
                        format!("{} bg-primary/10 text-primary font-medium", base)
                    } else {
                        format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                    }
                }>
                    <Icon glyph=Glyph::LayoutDashboard class="w-4 h-4" />"Dashboard"
                </button>
                <button on:click=move |_| open.update(|v| *v = !*v)
                    class="flex w-full items-center justify-between rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <div class="flex items-center gap-3">
                        <Icon glyph=Glyph::Folder class="w-4 h-4" />"Projects"
                    </div>
                    <Icon glyph=Glyph::ChevronDown class=move || {
                        if open.get() { "w-4 h-4 transition-transform rotate-180" } else { "w-4 h-4 transition-transform" }
                    } />
                </button>
                <Show when=move || open.get()>
                    <div class="ml-6 space-y-1 border-l border-border pl-3">
                        {vec!["Active", "Archived", "Templates"].into_iter().map(|item| {
                            let l = item;
                            let is_active = move || active.get() == l;
                            let click = move |_| active.set(l);
                            view! {
                                <button on:click=click class=move || {
                                    let base = "block w-full text-left rounded-md px-3 py-1.5 text-sm transition-colors";
                                    if is_active() {
                                        format!("{} bg-primary/10 text-primary font-medium", base)
                                    } else {
                                        format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                                    }
                                }>{item}</button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </Show>
                <button on:click=move |_| active.set("Settings") class=move || {
                    let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                    if active.get() == "Settings" {
                        format!("{} bg-primary/10 text-primary font-medium", base)
                    } else {
                        format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                    }
                }>
                    <Icon glyph=Glyph::Settings class="w-4 h-4" />"Settings"
                </button>
            </div>
        </div>
    }
}
