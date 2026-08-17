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
pub fn SidenavRoutesSimplified() -> impl IntoView {
    let active = RwSignal::new("Home");
    let routes = vec![
        ("Home", Glyph::LayoutDashboard),
        ("Docs", Glyph::BookOpen),
        ("Components", Glyph::Blocks),
        ("Pricing", Glyph::CreditCard),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-44 p-3 space-y-0.5">
                {routes.into_iter().map(|(label, icon)| {
                    let l = label;
                    let is_active = move || active.get() == l;
                    let click = move |_| active.set(l);
                    view! {
                        <button on:click=click class=move || {
                            let base = "flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs transition-colors";
                            if is_active() {
                                format!("{} bg-primary/10 text-primary font-medium", base)
                            } else {
                                format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                            }
                        }>
                            <Icon glyph=icon class="w-3.5 h-3.5" />
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
