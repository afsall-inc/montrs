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
pub fn Switch(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    let merged = move || {
        let base = "peer inline-flex h-[24px] w-[44px] shrink-0 \
                    cursor-pointer items-center rounded-full border-2 \
                    border-transparent transition-colors \
                    focus-visible:outline-none focus-visible:ring-2 \
                    focus-visible:ring-ring focus-visible:ring-offset-2 \
                    focus-visible:ring-offset-background \
                    disabled:cursor-not-allowed disabled:opacity-50";
        let state = if checked.get() {
            "bg-primary"
        } else {
            "bg-input"
        };
        cn!(base, state, class.get())
    };

    let toggle = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == " " {
            ev.prevent_default();
            checked.update(|v| *v = !*v);
        }
    };

    let click = move |_| checked.update(|v| *v = !*v);

    let id = crate::utils::Utils::use_random_id();

    let thumb_class = move || {
        let base = "pointer-events-none block h-5 w-5 rounded-full \
                    bg-background shadow-lg ring-0 transition-transform";
        let translate = if checked.get() {
            "translate-x-5"
        } else {
            "translate-x-0"
        };
        cn!(base, translate)
    };

    view! {
        <div class="flex items-center space-x-2">
            <button
                type="button"
                role="switch"
                id=id.clone()
                class=merged
                aria-checked=move || checked.get()
                aria-label=aria_label
                aria-disabled=disabled.then_some("true")
                data-state=move || if checked.get() { "checked" } else { "unchecked" }
                disabled=disabled
                on:click=click
                on:keydown=toggle
                data-name="Switch"
            >
                <span class=thumb_class data-name="SwitchThumb" />
            </button>
            {label.map(move |l| {
                let label_id = id.clone();
                view! {
                    <label
                        for=label_id
                        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                        {l}
                    </label>
                }
            })}
        </div>
    }
}
