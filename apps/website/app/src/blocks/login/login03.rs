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
pub fn Login03() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let e = email.get().trim().to_string();
        if e.is_empty() {
            error.set("Please enter your email.".to_string());
            return;
        }
        if !e.contains('@') {
            error.set("Invalid email.".to_string());
            return;
        }
        error.set(String::new());
        submitted.set(true);
    };

    let providers = vec![
        ("Continue with GitHub", Glyph::GitBranch),
        ("Continue with Google", Glyph::Globe),
        ("Continue with Discord", Glyph::MessageCircle),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm max-w-sm mx-auto">
            <div class="mb-6 text-center">
                <Icon glyph=Glyph::Blocks class="mx-auto w-10 h-10 text-primary" />
                <h3 class="mt-4 text-xl font-semibold">"Sign in"</h3>
            </div>
            <div class="space-y-3">
                {providers.into_iter().map(|(label, icon)| {
                    view! {
                        <button class="w-full flex items-center justify-center gap-2 rounded-md border border-border bg-background px-4 py-2 text-sm font-medium hover:bg-muted hover:border-primary/30 active:scale-[0.98] transition-all">
                            <Icon glyph=icon class="w-5 h-5" />
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="relative my-6">
                <div class="absolute inset-0 flex items-center"><span class="w-full border-t border-border" /></div>
                <div class="relative flex justify-center text-xs"><span class="bg-card px-2 text-muted-foreground">"Or continue with email"</span></div>
            </div>
            <form on:submit=submit class="space-y-4">
                <input
                    type="email"
                    placeholder="m@example.com"
                    on:input=move |ev| email.set(event_target_value(&ev))
                    prop:value=email
                    class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                />
                <Show when=move || !error.get().is_empty()>
                    <p class="text-xs text-red-500">{error}</p>
                </Show>
                <button type="submit" class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                    "Continue"
                </button>
            </form>
            <Show when=move || submitted.get()>
                <p class="mt-4 text-center text-sm text-green-600 dark:text-green-400">"Check your email for a sign-in link."</p>
            </Show>
        </div>
    }
}
