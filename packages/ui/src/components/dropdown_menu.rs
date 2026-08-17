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

#[derive(Clone)]
struct DropdownContext {
    open: RwSignal<bool>,
    focused_index: RwSignal<usize>,
    item_count: RwSignal<usize>,
}

#[component]
pub fn DropdownMenu(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    let focused_index = RwSignal::new(0usize);
    let item_count = RwSignal::new(0usize);

    provide_context(DropdownContext {
        open,
        focused_index,
        item_count,
    });

    view! {
        <div data-name="DropdownMenu">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DropdownContext>()
        .expect("DropdownMenuTrigger must be inside DropdownMenu");
    let toggle = move |_| {
        ctx.open.update(|v| *v = !*v);
        if ctx.open.get() {
            ctx.focused_index.set(0);
        }
    };

    let on_key_down =
        move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
            "ArrowDown" | "Enter" | " " => {
                ev.prevent_default();
                ctx.open.set(true);
                ctx.focused_index.set(0);
            }
            _ => {}
        };

    let merged = move || cn!("", class.get());

    view! {
        <button
            type="button"
            class=merged
            on:click=toggle
            on:keydown=on_key_down
            data-name="DropdownMenuTrigger"
            aria-haspopup="true"
            aria-expanded=move || ctx.open.get()
        >
            {children()}
        </button>
    }
}

fn focus_element_by_index(idx: usize) {
    let selector = format!("[data-dropdown-index=\"{}\"]", idx);
    if let Some(doc) = web_sys::window().and_then(|w| w.document())
        && let Ok(Some(el)) = doc.query_selector(&selector)
        && let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>()
    {
        let _ = html_el.focus();
    }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DropdownContext>()
        .expect("DropdownMenuContent must be inside DropdownMenu");

    let merged = move || {
        cn!(
            "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover \
             p-1 text-popover-foreground shadow-md \
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
                    focus_element_by_index(new_idx);
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
                    focus_element_by_index(new_idx);
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
            data-name="DropdownMenuContent"
            role="menu"
            aria-orientation="vertical"
            on:keydown=on_key_down
        >
            {children()}
        </div>
        {move || if ctx.open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="DropdownMenuBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DropdownContext>()
        .expect("DropdownMenuItem must be inside DropdownMenu");

    let idx = ctx.item_count.get();
    ctx.item_count.update(|c| *c += 1);

    let variant = if idx == 0 { Some(0) } else { None };

    let _ = variant;

    let merged = move || {
        cn!(
            "relative flex cursor-default select-none items-center rounded-sm \
             px-2 py-1.5 text-sm outline-none transition-colors \
             focus:bg-accent focus:text-accent-foreground \
             data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            class.get()
        )
    };

    let handle_click = move |_| {
        if let Some(cb) = on_select {
            cb.run(());
        }
        ctx.open.set(false);
    };

    let handle_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            if let Some(cb) = on_select {
                cb.run(());
            }
            ctx.open.set(false);
        }
    };

    view! {
        <div
            class=merged
            role="menuitem"
            tabindex="-1"
            data-dropdown-index=idx.to_string()
            on:click=handle_click
            on:keydown=handle_key_down
            data-name="DropdownMenuItem"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="DropdownMenuSeparator" role="separator" />
    }
}

#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("px-2 py-1.5 text-sm font-semibold", class.get());

    view! {
        <div class=merged data-name="DropdownMenuLabel" role="presentation">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuRadioGroup(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <div class=merged role="group" data-name="DropdownMenuRadioGroup">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuRadioItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DropdownContext>()
        .expect("DropdownMenuRadioItem must be inside DropdownMenu");

    let idx = ctx.item_count.get();
    ctx.item_count.update(|c| *c += 1);

    let merged = move || {
        cn!(
            "relative flex cursor-default select-none items-center rounded-sm \
             py-1.5 pl-8 pr-2 text-sm outline-none transition-colors \
             focus:bg-accent focus:text-accent-foreground \
             data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            class.get()
        )
    };

    let toggle = move |_| {
        checked.set(true);
        ctx.open.set(false);
    };

    let handle_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            checked.set(true);
            ctx.open.set(false);
        }
    };

    view! {
        <div
            class=merged
            role="menuitemradio"
            tabindex="-1"
            data-dropdown-index=idx.to_string()
            aria-checked=move || checked.get()
            on:click=toggle
            on:keydown=handle_key_down
            data-name="DropdownMenuRadioItem"
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {move || if checked.get() {
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
                            <circle cx="12" cy="12" r="2" />
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
pub fn DropdownMenuCheckboxItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DropdownContext>()
        .expect("DropdownMenuCheckboxItem must be inside DropdownMenu");

    let idx = ctx.item_count.get();
    ctx.item_count.update(|c| *c += 1);

    let merged = move || {
        cn!(
            "relative flex cursor-default select-none items-center rounded-sm \
             py-1.5 pl-8 pr-2 text-sm outline-none transition-colors \
             focus:bg-accent focus:text-accent-foreground \
             data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            class.get()
        )
    };

    let toggle = move |_| {
        checked.update(|v| *v = !*v);
    };

    let handle_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            checked.update(|v| *v = !*v);
        }
    };

    view! {
        <div
            class=merged
            role="menuitemcheckbox"
            tabindex="-1"
            data-dropdown-index=idx.to_string()
            aria-checked=move || checked.get()
            on:click=toggle
            on:keydown=handle_key_down
            data-name="DropdownMenuCheckboxItem"
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {move || if checked.get() {
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
pub fn DropdownMenuShortcut(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "ml-auto text-xs tracking-widest text-muted-foreground",
            class.get()
        )
    };

    view! {
        <span class=merged data-name="DropdownMenuShortcut">
            {children()}
        </span>
    }
}
