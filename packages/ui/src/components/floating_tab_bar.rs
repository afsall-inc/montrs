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

/// A centered, pill-shaped tab bar that floats just below the sticky header
/// (`h-14`), or renders inline when `sticky = false`.
///
/// Two common uses:
///
/// * in-page section navigation (buttons that scroll / route on select), and
/// * the persistent `/ui` sub-navigation.
///
/// `items` is a list of `(label, id)` pairs; `active` holds the id of the
/// selected tab and `on_select` is called with the id that was activated.
#[component]
pub fn FloatingTabBar(
    #[prop(into, optional)] class: Signal<String>,
    /// Float below the header while scrolling. Defaults to `true`.
    #[prop(default = true)]
    sticky: bool,
    items: Vec<(String, String)>,
    #[prop(into)] active: Signal<String>,
    on_select: Callback<String>,
) -> impl IntoView {
    let container_class = if sticky {
        "sticky top-16 z-30 flex justify-center py-2"
    } else {
        "flex justify-center py-2"
    };

    view! {
        <div class=container_class>
            <div
                class={move || {
                    cn!(
                        "inline-flex max-w-full items-center gap-0.5 overflow-x-auto rounded-full border border-border bg-background/80 p-1 shadow-sm backdrop-blur supports-[backdrop-filter]:bg-background/60",
                        class.get()
                    )
                }}
                role="tablist"
            >
                {items.into_iter().map(|(label, id)| {
                    let id_active = id.clone();
                    let id_class = id.clone();
                    let id_click = id.clone();
                    let on_click = on_select;
                    view! {
                        <button
                            type="button"
                            role="tab"
                            aria-selected=move || active.get() == id_active
                            class=move || {
                                cn!(
                                    "inline-flex shrink-0 items-center rounded-full px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-all",
                                    if active.get() == id_class {
                                        "bg-primary text-primary-foreground shadow"
                                    } else {
                                        "text-muted-foreground hover:bg-accent hover:text-foreground"
                                    }
                                )
                            }
                            on:click=move |_| on_click.run(id_click.clone())
                        >
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
