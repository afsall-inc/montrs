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
pub fn RadioButton(
    value: String,
    #[prop(into, optional)] _name: Option<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] selected: RwSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    let value_for_merged = value.clone();
    let merged = move || {
        let base = "aspect-square h-4 w-4 rounded-full border border-primary \
                    text-primary ring-offset-background focus:outline-none \
                    focus-visible:ring-2 focus-visible:ring-ring \
                    focus-visible:ring-offset-2 disabled:cursor-not-allowed \
                    disabled:opacity-50";
        let checked = if selected.get() == value_for_merged {
            "bg-primary text-primary-foreground"
        } else {
            ""
        };
        cn!(base, checked, class.get())
    };

    let id = crate::utils::Utils::use_random_id();
    let value_for_is_selected = value.clone();
    let is_selected = move || selected.get() == value_for_is_selected;
    let value_for_select = value.clone();
    let select = move |_| selected.set(value_for_select.clone());

    let on_key_down =
        move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
            "ArrowDown" | "ArrowRight" | "ArrowUp" | "ArrowLeft" => {
                ev.prevent_default();
            }
            _ => {}
        };

    let is_selected_for_aria = is_selected.clone();
    let is_selected_for_data_state = is_selected.clone();
    let is_selected_for_svg = is_selected.clone();

    view! {
        <div class="flex items-center space-x-2">
            <button
                type="button"
                role="radio"
                id=id.clone()
                class=merged
                aria-checked=is_selected_for_aria
                aria-label=aria_label
                data-state=move || if is_selected_for_data_state() { "checked" } else { "unchecked" }
                disabled=disabled
                on:click=select
                on:keydown=on_key_down
                data-name="RadioButton"
                value=value
            >
                {move || if is_selected_for_svg() {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24" height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="h-2.5 w-2.5 fill-current text-current"
                        >
                            <circle cx="12" cy="12" r="2" />
                        </svg>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
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

#[component]
pub fn RadioGroup(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("grid gap-2", class.get());

    let on_key_down =
        move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
            "ArrowDown" | "ArrowRight" | "ArrowUp" | "ArrowLeft" => {
                ev.prevent_default();
            }
            _ => {}
        };

    view! {
        <div
            class=merged
            role="radiogroup"
            data-name="RadioGroup"
            on:keydown=on_key_down
        >
            {children()}
        </div>
    }
}
