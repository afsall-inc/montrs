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
pub fn DatePickerDualState(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] start: RwSignal<String>,
    #[prop(into, optional)] end: RwSignal<String>,
) -> impl IntoView {
    let merged = move || cn!("flex items-center gap-2", class.get());
    let on_start =
        move |ev: leptos::ev::Event| start.set(event_target_value(&ev));
    let on_end = move |ev: leptos::ev::Event| end.set(event_target_value(&ev));
    view! {
        <div class=merged data-name="DatePickerDualState">
            <input type="date" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value=move || start.get() on:input=on_start />
            <span class="text-muted-foreground">"to"</span>
            <input type="date" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value=move || end.get() on:input=on_end />
        </div>
    }
}
