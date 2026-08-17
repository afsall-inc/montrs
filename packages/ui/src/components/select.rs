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
use leptos::{prelude::*, wasm_bindgen::JsCast};

crate::variants! {
    Select {
        base: "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
        variants: {
            variant: {
                Default: "",
            }
        }
    }
}

#[derive(Clone)]
struct SelectContext {
    open: RwSignal<bool>,
    value: RwSignal<String>,
    focused_index: RwSignal<usize>,
    item_count: RwSignal<usize>,
}

#[component]
pub fn Select(
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] _placeholder: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let focused_index = RwSignal::new(0usize);
    let item_count = RwSignal::new(0usize);

    provide_context(SelectContext {
        open,
        value,
        focused_index,
        item_count,
    });

    let merged = move || {
        let v = SelectVariant::Default;
        let c = SelectClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };

    let toggle = move |_| {
        open.update(|v| *v = !*v);
        if open.get() {
            focused_index.set(0);
        }
    };

    let on_key_down =
        move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
            "ArrowDown" | "Enter" | " " => {
                ev.prevent_default();
                open.set(true);
                focused_index.set(0);
            }
            "Escape" => {
                ev.prevent_default();
                open.set(false);
            }
            _ => {}
        };

    view! {
        <div class="relative" data-name="Select">
            <button
                type="button"
                role="combobox"
                class=merged
                aria-expanded=move || open.get()
                aria-haspopup="listbox"
                aria-label="Select"
                on:click=toggle
                on:keydown=on_key_down
                data-name="SelectTrigger"
            >
                <span>{move || value.get().to_string()}</span>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24" height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-4 w-4 opacity-50"
                >
                    <path d="m6 9 6 6 6-6" />
                </svg>
            </button>
            <SelectContent>
                {children()}
            </SelectContent>
        </div>
    }
}

fn focus_select_item_by_index(idx: usize) {
    let selector = format!("[data-select-index=\"{}\"]", idx);
    if let Some(doc) = web_sys::window().and_then(|w| w.document())
        && let Ok(Some(el)) = doc.query_selector(&selector)
        && let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>()
    {
        let _ = html_el.focus();
    }
}

#[component]
pub fn SelectContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>()
        .expect("SelectContent must be inside Select");

    let merged = move || {
        cn!(
            "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md \
             border bg-popover text-popover-foreground shadow-md \
             data-[state=open]:animate-in data-[state=closed]:animate-out \
             data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
             data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
            class.get()
        )
    };

    let close = move |_| ctx.open.set(false);

    let on_key_down =
        move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let count = ctx.item_count.get();
                if count > 0 {
                    let new_idx = (ctx.focused_index.get() + 1) % count;
                    ctx.focused_index.set(new_idx);
                    focus_select_item_by_index(new_idx);
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                let count = ctx.item_count.get();
                if count > 0 {
                    let new_idx = if ctx.focused_index.get() == 0 {
                        count - 1
                    } else {
                        ctx.focused_index.get() - 1
                    };
                    ctx.focused_index.set(new_idx);
                    focus_select_item_by_index(new_idx);
                }
            }
            "Escape" => {
                ev.prevent_default();
                ctx.open.set(false);
            }
            _ => {}
        };

    view! {
        <div
            class=merged
            data-state=move || if ctx.open.get() { "open" } else { "closed" }
            hidden=move || !ctx.open.get()
            data-name="SelectContent"
            role="listbox"
            aria-label="Options"
            on:keydown=on_key_down
        >
            <div class="max-h-96 overflow-y-auto">
                {children()}
            </div>
        </div>
        {move || if ctx.open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="SelectBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

#[component]
pub fn SelectItem(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>()
        .expect("SelectItem must be inside Select");

    let idx = ctx.item_count.get();
    ctx.item_count.update(|c| *c += 1);

    let value_for_is_selected = value.clone();
    let is_selected = move || ctx.value.get() == value_for_is_selected;
    let is_selected_for_merged = is_selected.clone();
    let merged = move || {
        let base = "relative flex w-full cursor-default select-none \
                    items-center rounded-sm py-1.5 pl-8 pr-2 text-sm \
                    outline-none focus:bg-accent focus:text-accent-foreground \
                    data-[disabled]:pointer-events-none \
                    data-[disabled]:opacity-50";
        let active = if is_selected_for_merged() {
            "bg-accent text-accent-foreground"
        } else {
            ""
        };
        cn!(base, active, class.get())
    };

    let value_for_select = value.clone();
    let value_for_kd = value.clone();
    let select = move |_| {
        ctx.value.set(value_for_select.clone());
        ctx.open.set(false);
    };

    let handle_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            ctx.value.set(value_for_kd.clone());
            ctx.open.set(false);
        }
    };

    let is_selected_for_aria = is_selected.clone();
    let is_selected_for_svg = is_selected.clone();

    view! {
        <div
            class=merged
            role="option"
            tabindex="-1"
            data-select-index=idx.to_string()
            aria-selected=is_selected_for_aria
            on:click=select
            on:keydown=handle_key_down
            data-name="SelectItem"
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {move || if is_selected_for_svg() {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24" height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="h-4 w-4"
                        >
                            <path d="M20 6 9 17l-5-5" />
                        </svg>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </span>
            {children()}
        </div>
    }
}

#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="SelectSeparator" role="separator" />
    }
}

#[component]
pub fn SelectLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged =
        move || cn!("py-1.5 pl-8 pr-2 text-sm font-semibold", class.get());

    view! {
        <div class=merged data-name="SelectLabel" role="presentation">
            {children()}
        </div>
    }
}
