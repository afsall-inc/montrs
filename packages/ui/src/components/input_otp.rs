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
pub fn InputOtp(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(optional)] length: u8,
) -> impl IntoView {
    let len = length.clamp(4, 8) as usize;
    let merged = move || cn!("flex items-center gap-2", class.get());
    let chars = move || {
        let v = value.get();
        let chars: Vec<char> = v.chars().collect();
        (0..len)
            .map(move |i| chars.get(i).copied().unwrap_or(' '))
            .collect::<Vec<_>>()
    };
    let on_input = move |ev: leptos::ev::Event| {
        let val: String = event_target_value(&ev)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .take(len)
            .collect();
        value.set(val);
    };
    view! {
        <div class=merged data-name="InputOtp">
            <input
                type="text"
                inputmode="numeric"
                maxlength=len.to_string()
                class="sr-only"
                value=move || value.get()
                on:input=on_input
            />
            {move || chars().into_iter().map(|c| {
                view! {
                    <div class="flex h-12 w-10 items-center justify-center rounded-md border border-input text-sm font-mono bg-background">
                        {if c != ' ' { c.to_string() } else { String::new() }}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
