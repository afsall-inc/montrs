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
pub fn Footer05() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let subscribed = RwSignal::new(false);
    let subscribe = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !email.get().trim().is_empty() {
            subscribed.set(true);
        }
    };

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div class="flex flex-col md:flex-row items-center justify-between gap-4">
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                    <span class="text-sm font-semibold">"MontRS"</span>
                </div>
                <form on:submit=subscribe class="flex items-center gap-2">
                    <input
                        type="email"
                        placeholder="Email"
                        on:input=move |ev| email.set(event_target_value(&ev))
                        prop:value=email
                        class="w-40 rounded-md border border-input bg-background px-3 py-1.5 text-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                    <button type="submit" class="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                        "Go"
                    </button>
                </form>
            </div>
            <Show when=move || subscribed.get()>
                <p class="mt-2 text-center text-xs text-green-600 dark:text-green-400">"Subscribed!"</p>
            </Show>
            <div class="flex flex-col md:flex-row items-center justify-between gap-4 mt-4 pt-4 border-t border-border">
                <div class="flex items-center gap-4 text-sm text-muted-foreground">
                    <a href="#" class="hover:text-foreground hover:scale-105 inline-block transition-all">"Privacy"</a>
                    <a href="#" class="hover:text-foreground hover:scale-105 inline-block transition-all">"Terms"</a>
                    <a href="#" class="hover:text-foreground hover:scale-105 inline-block transition-all">"Contact"</a>
                </div>
                <p class="text-xs text-muted-foreground">"© 2026 MontRS"</p>
            </div>
        </div>
    }
}
