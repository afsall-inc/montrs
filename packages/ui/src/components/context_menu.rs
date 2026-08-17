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

/// Context menu triggered by right-click.
///
/// Provides a floating menu at the cursor position.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <ContextMenu>
///         <ContextMenuTrigger>"Right-click me"</ContextMenuTrigger>
///         <ContextMenuContent>
///             <ContextMenuItem>"Edit"</ContextMenuItem>
///             <ContextMenuSeparator />
///             <ContextMenuItem>"Delete"</ContextMenuItem>
///         </ContextMenuContent>
///     </ContextMenu>
/// }
/// ```
#[component]
pub fn ContextMenu(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    let position = RwSignal::new((0.0, 0.0));
    provide_context(open);
    provide_context(position);

    view! {
        <div data-name="ContextMenu">
            {children()}
        </div>
    }
}

/// Trigger element that shows the context menu on right-click.
#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("ContextMenuTrigger must be inside ContextMenu");
    let position = use_context::<RwSignal<(f64, f64)>>()
        .expect("ContextMenuTrigger must be inside ContextMenu");

    let on_context_menu = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        open.set(true);
        position.set((ev.client_x() as f64, ev.client_y() as f64));
    };

    let merged = move || cn!("", class.get());

    view! {
        <div class=merged on:contextmenu=on_context_menu data-name="ContextMenuTrigger">
            {children()}
        </div>
    }
}

/// Context menu content positioned at cursor.
#[component]
pub fn ContextMenuContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("ContextMenuContent must be inside ContextMenu");
    let position = use_context::<RwSignal<(f64, f64)>>()
        .expect("ContextMenuContent must be inside ContextMenu");

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

    let style = move || {
        let (x, y) = position.get();
        format!("left:{}px; top:{}px; position:fixed;", x, y)
    };

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            style=style
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="ContextMenuContent"
        >
            {children()}
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="ContextMenuBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

/// Context menu item.
#[component]
pub fn ContextMenuItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "relative flex cursor-default select-none items-center rounded-sm \
             px-2 py-1.5 text-sm outline-none focus:bg-accent \
             focus:text-accent-foreground data-[disabled]:pointer-events-none \
             data-[disabled]:opacity-50",
            class.get()
        )
    };

    let handle_click = move |_| {
        if let Some(cb) = on_select {
            cb.run(());
        }
    };

    view! {
        <div
            class=merged
            role="menuitem"
            on:click=handle_click
            data-name="ContextMenuItem"
        >
            {children()}
        </div>
    }
}

/// Context menu separator.
#[component]
pub fn ContextMenuSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="ContextMenuSeparator" />
    }
}
