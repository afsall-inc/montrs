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
pub fn Header() -> impl IntoView {
    let theme = use_theme();

    view! {
        <header class="sticky top-0 z-50 w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="mx-auto max-w-6xl flex h-16 items-center justify-between px-6 lg:px-8">
                <div class="flex items-center gap-6">
                    <a href="/" class="flex items-center gap-2 font-bold text-lg">
                        <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                        "MontRS"
                    </a>
                    <nav class="hidden md:flex items-center gap-6 text-sm">
                        <a href="/" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Home"
                        </a>
                        <a href="/icons" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Icons"
                        </a>
                        <a href="/components" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Components"
                        </a>
                        <a href="/blocks" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Blocks"
                        </a>
                        <a href="/motion" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Motion"
                        </a>
                        <a href="/animated-icons" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Animated Icons"
                        </a>
                    </nav>
                </div>
                <div class="flex items-center gap-4">
                    <a href="https://github.com/montrs/montrs" target="_blank"
                        class="text-muted-foreground hover:text-foreground transition-colors"
                    >
                        <Icon glyph=Glyph::Globe class="w-5 h-5" />
                    </a>
                    <button
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 w-9"
                        on:click=move |_| toggle_theme()
                        aria-label="Toggle theme"
                    >
                        {move || match theme.get() {
                            ThemeMode::Light => view! { <Icon glyph=Glyph::Sun class="w-4 h-4" /> }.into_any(),
                            ThemeMode::Dark => view! { <Icon glyph=Glyph::Moon class="w-4 h-4" /> }.into_any(),
                            ThemeMode::System => view! { <Icon glyph=Glyph::Monitor class="w-4 h-4" /> }.into_any(),
                        }}
                    </button>
                </div>
            </div>
        </header>
    }
}
