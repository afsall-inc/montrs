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

use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn FaqTransition(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged =
        move || cn!("divide-y divide-border rounded-lg border", class.get());
    view! {
        <div class=merged data-name="FaqTransition">
            {children()}
        </div>
    }
}

#[component]
pub fn FaqItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] question: Option<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || cn!("", class.get());
    let toggle = move |_| open.update(|v| *v = !*v);
    let question_text = question.clone();
    view! {
        <div class=merged data-name="FaqItem">
            <button
                type="button"
                class="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium hover:bg-muted/50"
                on:click=toggle
            >
                {question_text.map(|q| view! { <span>{q}</span> })}
                <svg
                    xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                    viewBox="0 0 24 24" fill="none" stroke="currentColor"
                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                    class=move || if open.get() { "rotate-180 transition-transform" } else { "transition-transform" }
                >
                    <path d="m6 9 6 6 6-6" />
                </svg>
            </button>
            <div class=move || {
                let base = "overflow-hidden transition-all duration-200";
                let state = if open.get() { "max-h-96 px-4 pb-3" } else { "max-h-0" };
                cn!(base, state)
            }>
                {children()}
            </div>
        </div>
    }
}
