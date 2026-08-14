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
pub fn Tooltip(
    #[prop(into, optional)] text: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || cn!("relative inline-flex", class.get());

    let tooltip_id = crate::utils::Utils::use_random_id();

    let show = move |_| open.set(true);
    let hide = move |_| open.set(false);

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            open.set(false);
        }
    };

    let on_focus = move |_| open.set(true);
    let on_blur = move |_| open.set(false);

    view! {
        <div
            class=merged
            on:mouseenter=show
            on:mouseleave=hide
            on:focusin=on_focus
            on:focusout=on_blur
            on:keydown=on_key_down
            data-name="Tooltip"
        >
            <div
                aria-describedby=tooltip_id.clone()
                data-name="TooltipTrigger"
            >
                {children()}
            </div>
            <div
                role="tooltip"
                id=tooltip_id.clone()
                class="z-50 overflow-hidden rounded-md border bg-popover px-3 py-1.5 text-sm text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95"
                data-state=move || if open.get() { "open" } else { "closed" }
                hidden=move || !open.get()
                data-name="TooltipContent"
            >
                {text.clone()}
            </div>
        </div>
    }
}
