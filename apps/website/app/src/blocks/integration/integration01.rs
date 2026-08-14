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
pub fn Integration01() -> impl IntoView {
    let copied = RwSignal::new(Option::<String>::None);
    let icons = vec![
        Glyph::Search,
        Glyph::Settings,
        Glyph::User,
        Glyph::Bell,
        Glyph::LayoutDashboard,
        Glyph::Mail,
        Glyph::Calendar,
        Glyph::Clock,
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <h3 class="text-sm font-semibold mb-4">"Icon Library — Click to copy name"</h3>
            <div class="grid grid-cols-4 gap-4">
                {icons.into_iter().map(|g| {
                    let name = format!("{:?}", g);
                    let name_clone = name.clone();
                    let name_for_copied = name.clone();
                    let click = move |_| {
                        copied.set(Some(name_clone.clone()));
                    };
                    let is_copied = move || copied.get().as_deref() == Some(&name_for_copied);
                    view! {
                        <button on:click=click class="flex flex-col items-center gap-2 rounded-lg border border-border bg-muted/50 p-4 hover:bg-muted hover:border-primary/30 active:scale-95 transition-all">
                            <Icon glyph=g class="w-6 h-6 text-foreground" />
                            <span class="text-xs text-muted-foreground">{name}</span>
                            <Show when=is_copied>
                                <span class="text-[10px] text-green-500">"Copied!"</span>
                            </Show>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
