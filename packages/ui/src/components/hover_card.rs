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

/// Hover card / tooltip that appears on hover.
///
/// Shows additional content when hovering over a trigger element.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <HoverCard>
///         <HoverCardTrigger>"Hover me"</HoverCardTrigger>
///         <HoverCardContent>"Extra info"</HoverCardContent>
///     </HoverCard>
/// }
/// ```
#[component]
pub fn HoverCard(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="HoverCard">
            {children()}
        </div>
    }
}

/// Trigger element that shows the hover card.
#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("HoverCardTrigger must be inside HoverCard");

    let merged = move || cn!("", class.get());

    view! {
        <div
            class=merged
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
            data-name="HoverCardTrigger"
        >
            {children()}
        </div>
    }
}

/// Hover card content.
#[component]
pub fn HoverCardContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("HoverCardContent must be inside HoverCard");

    let merged = move || {
        cn!(
            "z-50 w-64 rounded-md border bg-popover p-4 \
             text-popover-foreground shadow-md outline-none \
             data-[state=open]:animate-in data-[state=closed]:animate-out \
             data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
             data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
            class.get()
        )
    };

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="HoverCardContent"
        >
            {children()}
        </div>
    }
}
