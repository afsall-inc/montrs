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
pub fn MultiSelect(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] value: RwSignal<Vec<String>>,
    _children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || {
        cn!(
            "flex h-10 w-full items-center justify-between rounded-md border \
             border-input bg-background px-3 py-2 text-sm \
             ring-offset-background",
            class.get()
        )
    };
    let toggle = move |_| open.update(|v| *v = !*v);
    view! {
        <div class="relative" data-name="MultiSelect">
            <button type="button" class=merged on:click=toggle data-name="MultiSelectTrigger">
                <span>{move || format!("{} selected", value.get().len())}</span>
            </button>
        </div>
    }
}

#[component]
pub fn MultiSelectItem(
    _value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex cursor-pointer items-center gap-2 rounded-sm px-2 py-1.5 \
             text-sm hover:bg-accent",
            class.get()
        )
    };
    view! {
        <div class=merged data-name="MultiSelectItem">
            {children()}
        </div>
    }
}
