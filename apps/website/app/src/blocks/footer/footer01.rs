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
pub fn Footer01() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let subscribed = RwSignal::new(false);
    let subscribe = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !email.get().trim().is_empty() {
            subscribed.set(true);
        }
    };

    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm text-center">
            <div class="flex items-center justify-center gap-2 mb-4">
                <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                <span class="font-bold text-lg">"MontRS"</span>
            </div>
            <p class="text-sm text-muted-foreground mb-6">"Building the future of full-stack Rust web development."</p>
            <form on:submit=subscribe class="flex items-center justify-center gap-2 mb-6">
                <input
                    type="email"
                    placeholder="Enter your email"
                    on:input=move |ev| email.set(event_target_value(&ev))
                    prop:value=email
                    class="max-w-xs w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                />
                <button type="submit" class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors shrink-0">
                    "Subscribe"
                </button>
            </form>
            <Show when=move || subscribed.get()>
                <p class="text-xs text-green-600 dark:text-green-400 mb-4">"Thanks for subscribing!"</p>
            </Show>
            <div class="flex justify-center gap-4 text-sm text-muted-foreground">
                <a href="#" class="hover:text-foreground transition-colors hover:scale-105 inline-block">"Twitter"</a>
                <a href="#" class="hover:text-foreground transition-colors hover:scale-105 inline-block">"GitHub"</a>
                <a href="#" class="hover:text-foreground transition-colors hover:scale-105 inline-block">"Discord"</a>
            </div>
            <p class="mt-6 text-xs text-muted-foreground">"© 2026 MontRS. All rights reserved."</p>
        </div>
    }
}
