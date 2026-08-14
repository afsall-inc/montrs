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

/// Multi-line text input component.
///
/// Renders a styled textarea with label, description, and error state.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Textarea
///         label="Description"
///         placeholder="Enter description..."
///         value=description_signal
///     />
/// }
/// ```
#[component]
pub fn Textarea(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, optional)] error: Option<String>,
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] rows: u32,
) -> impl IntoView {
    let input_id = id.unwrap_or_else(crate::utils::Utils::use_random_id);
    let error_for_merged = error.clone();
    let merged = move || {
        let base = "flex min-h-[80px] w-full rounded-md border border-input \
                    bg-background px-3 py-2 text-sm ring-offset-background \
                    placeholder:text-muted-foreground \
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
            <textarea
                id=input_id
                class=merged
                placeholder=placeholder
                prop:value=move || value.get()
                disabled=disabled
                required=required
                rows=rows
                on:input=on_input
                data-name="Textarea"
            ></textarea>
            {description.map(|d| view! {
                <p class="text-sm text-muted-foreground">{d}</p>
            })}
            {error.filter(|e| !e.is_empty()).map(|e| view! {
                <p class="text-sm font-medium text-destructive">{e}</p>
            })}
        </div>
    }
}
