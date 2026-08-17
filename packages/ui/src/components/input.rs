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
pub fn Input(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, optional)] error: Option<String>,
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] input_type: &'static str,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] readonly: bool,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    let input_id = id.unwrap_or_else(crate::utils::Utils::use_random_id);
    let error_id = crate::utils::Utils::use_random_id();
    let error_for_merged = error.clone();
    let error_for_has_error = error.clone();
    let error_for_input = error.clone();
    let error_for_display = error;
    let merged = move || {
        let base = "flex h-10 w-full rounded-md border border-input \
                    bg-background px-3 py-2 text-sm ring-offset-background \
                    file:border-0 file:bg-transparent file:text-sm \
                    file:font-medium placeholder:text-muted-foreground \
                    focus-visible:outline-none focus-visible:ring-2 \
                    focus-visible:ring-ring focus-visible:ring-offset-2 \
                    disabled:cursor-not-allowed disabled:opacity-50";
        let error_class =
            if error_for_merged.as_ref().is_some_and(|e| !e.is_empty()) {
                "border-destructive"
            } else {
                ""
            };
        cn!(base, error_class, class.get())
    };

    let on_input = move |ev: leptos::ev::Event| {
        let target = event_target_value(&ev);
        value.set(target);
    };

    let has_error =
        move || error_for_has_error.as_ref().is_some_and(|e| !e.is_empty());

    view! {
        <div class="grid gap-1.5">
            {label.map({
                let input_id = input_id.clone();
                move |l| {
                    let label_id = input_id.clone();
                    view! {
                        <label
                            for=label_id
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                        >
                            {l}
                        </label>
                    }
                }
            })}
            <input
                id=input_id
                type=input_type
                class=merged
                placeholder=placeholder
                value=move || value.get()
                disabled=disabled
                required=required
                readonly=readonly
                aria-invalid=move || has_error().then_some("true")
                aria-describedby=error_for_input.as_ref().filter(|e| !e.is_empty()).map(|_| error_id.clone())
                aria-required=required.then_some("true")
                aria-label=aria_label
                aria-disabled=disabled.then_some("true")
                on:input=on_input
                data-name="Input"
            />
            {description.map(|d| view! {
                <p class="text-sm text-muted-foreground">{d}</p>
            })}
            {error_for_display.filter(|e| !e.is_empty()).map(move |e| {
                let error_id = error_id.clone();
                view! {
                    <p class="text-sm font-medium text-destructive" id=error_id role="alert">{e}</p>
                }
            })}
        </div>
    }
}
