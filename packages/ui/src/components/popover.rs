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

/// Popover with trigger and content.
///
/// Shows a floating content panel when the trigger is clicked.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Popover>
///         <PopoverTrigger>"Open"</PopoverTrigger>
///         <PopoverContent>"Popover content"</PopoverContent>
///     </Popover>
/// }
/// ```
#[component]
pub fn Popover(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="Popover">
            {children()}
        </div>
    }
}

/// Trigger that opens the popover.
#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("PopoverTrigger must be inside Popover");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || cn!("", class.get());

    view! {
        <button type="button" class=merged on:click=toggle data-name="PopoverTrigger">
            {children()}
        </button>
    }
}

/// Popover content panel.
#[component]
pub fn PopoverContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("PopoverContent must be inside Popover");

    let merged = move || {
        cn!(
            "z-50 w-72 rounded-md border bg-popover p-4 \
             text-popover-foreground shadow-md outline-none \
             data-[state=open]:animate-in data-[state=closed]:animate-out \
             data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
             data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
            class.get()
        )
    };

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="PopoverContent"
        >
            {children()}
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="PopoverBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}
