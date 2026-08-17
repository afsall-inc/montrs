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
pub fn Sheet(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] open: bool,
    #[prop(into, optional)] _on_close: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "fixed inset-y-0 right-0 z-50 flex w-full max-w-md \
                    flex-col border-l bg-background shadow-xl \
                    transition-transform";
        let state = if open {
            "translate-x-0"
        } else {
            "translate-x-full"
        };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Sheet" hidden=!open>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex items-center justify-between border-b px-6 py-4",
            class.get()
        )
    };
    view! {
        <div class=merged data-name="SheetHeader">
            {children()}
        </div>
    }
}

#[component]
pub fn SheetContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex-1 overflow-y-auto px-6 py-4", class.get());
    view! {
        <div class=merged data-name="SheetContent">
            {children()}
        </div>
    }
}

#[component]
pub fn SheetFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex items-center justify-end gap-2 border-t px-6 py-4",
            class.get()
        )
    };
    view! {
        <div class=merged data-name="SheetFooter">
            {children()}
        </div>
    }
}
