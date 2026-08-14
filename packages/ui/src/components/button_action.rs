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
pub fn ButtonAction(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] loading: bool,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "inline-flex items-center justify-center gap-2 \
                    whitespace-nowrap rounded-md text-sm font-medium \
                    ring-offset-background transition-colors \
                    focus-visible:outline-none focus-visible:ring-2 \
                    focus-visible:ring-ring focus-visible:ring-offset-2 \
                    disabled:pointer-events-none disabled:opacity-50 \
                    bg-primary text-primary-foreground hover:bg-primary/90 \
                    h-10 px-4 py-2";
        cn!(base, class.get())
    };
    let handle_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };
    view! {
        <button
            type="button"
            class=merged
            disabled=disabled || loading
            on:click=handle_click
            data-name="ButtonAction"
        >
            {children()}
        </button>
    }
}
