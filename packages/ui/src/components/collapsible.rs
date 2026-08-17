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

/// Collapsible section with a trigger that toggles content visibility.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Collapsible>
///         <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
///         <CollapsibleContent>"Hidden content"</CollapsibleContent>
///     </Collapsible>
/// }
/// ```
#[component]
pub fn Collapsible(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] default_open: bool,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    provide_context(open);

    let merged = move || cn!("", class.get());

    view! {
        <div class=merged data-name="Collapsible">
            {children()}
        </div>
    }
}

/// Trigger button that toggles collapsible content.
#[component]
pub fn CollapsibleTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("CollapsibleTrigger must be inside Collapsible");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || {
        cn!(
            "flex w-full items-center justify-between py-2 text-sm \
             font-medium [&[data-state=open]>svg]:rotate-180",
            class.get()
        )
    };

    let state = move || if open.get() { "open" } else { "closed" };

    view! {
        <button type="button" class=merged data-state=state on:click=toggle data-name="CollapsibleTrigger">
            {children()}
        </button>
    }
}

/// Content area that expands/collapses.
#[component]
pub fn CollapsibleContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("CollapsibleContent must be inside Collapsible");

    let merged = move || {
        cn!(
            "overflow-hidden data-[state=closed]:animate-collapsible-up \
             data-[state=open]:animate-collapsible-down",
            class.get()
        )
    };

    let state = move || if open.get() { "open" } else { "closed" };

    view! {
        <div
            class=merged
            data-state=state
            hidden=move || !open.get()
            data-name="CollapsibleContent"
        >
            {children()}
        </div>
    }
}
