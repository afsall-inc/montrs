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
pub fn Form(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_submit: Option<
        Callback<leptos::ev::SubmitEvent>,
    >,
    #[prop(optional)] novalidate: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("space-y-6", class.get());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        if novalidate {
            ev.prevent_default();
        }
        if let Some(cb) = on_submit {
            cb.run(ev);
        }
    };

    view! {
        <form
            class=merged
            data-name="Form"
            role="form"
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            novalidate=novalidate.then_some("")
            on:submit=handle_submit
        >
            {children()}
        </form>
    }
}

#[component]
pub fn FormField(
    name: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("space-y-2", class.get());

    view! {
        <div class=merged data-name="FormField" data-field-name=name>
            {children()}
        </div>
    }
}

#[component]
pub fn FormLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "text-sm font-medium leading-none \
             peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
            class.get()
        )
    };

    view! {
        <label class=merged data-name="FormLabel">
            {children()}
        </label>
    }
}

#[component]
pub fn FormControl(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <div class=merged data-name="FormControl">
            {children()}
        </div>
    }
}

#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <p class=merged data-name="FormDescription" id=crate::utils::Utils::use_random_id()>
            {children()}
        </p>
    }
}

#[component]
pub fn FormMessage(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged =
        move || cn!("text-sm font-medium text-destructive", class.get());

    let id = crate::utils::Utils::use_random_id();

    view! {
        <p
            class=merged
            data-name="FormMessage"
            id=id
            role="alert"
            aria-live="polite"
        >
            {children()}
        </p>
    }
}
