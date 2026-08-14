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

/// Menu bar with items.
///
/// Renders a horizontal menu bar with dropdown triggers.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Menubar>
///         <MenubarMenu>
///             <MenubarTrigger>"File"</MenubarTrigger>
///             <MenubarContent>
///                 <MenubarItem>"New"</MenubarItem>
///             </MenubarContent>
///         </MenubarMenu>
///     </Menubar>
/// }
/// ```
#[component]
pub fn Menubar(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex h-10 items-center space-x-1 rounded-md border bg-background \
             p-1",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="Menubar">
            {children()}
        </div>
    }
}

/// Individual menu in the menubar.
#[component]
pub fn MenubarMenu(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="MenubarMenu">
            {children()}
        </div>
    }
}

/// Trigger button for a menu.
#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("MenubarTrigger must be inside MenubarMenu");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || {
        cn!(
            "flex cursor-default select-none items-center rounded-sm px-3 \
             py-1.5 text-sm font-medium outline-none focus:bg-accent \
             focus:text-accent-foreground data-[state=open]:bg-accent \
             data-[state=open]:text-accent-foreground",
            class.get()
        )
    };

    view! {
        <button type="button" class=merged on:click=toggle data-name="MenubarTrigger">
            {children()}
        </button>
    }
}

/// Menu content dropdown.
#[component]
pub fn MenubarContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("MenubarContent must be inside MenubarMenu");

    let merged = move || {
        cn!(
            "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover \
             p-1 text-popover-foreground shadow-md \
             data-[state=open]:animate-in data-[state=closed]:fade-out-0 \
             data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 \
             data-[state=open]:zoom-in-95",
            class.get()
        )
    };

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="MenubarContent"
        >
            {children()}
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="MenubarBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

/// Menubar item.
#[component]
pub fn MenubarItem(
    #[prop(into, optional)] class: Signal<String>,
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

    view! {
        <div class=merged role="menuitem" data-name="MenubarItem">
            {children()}
        </div>
    }
}

/// Menubar separator.
#[component]
pub fn MenubarSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="MenubarSeparator" />
    }
}

/// Menubar shortcut.
#[component]
pub fn MenubarShortcut(
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
        <span class=merged data-name="MenubarShortcut">
            {children()}
        </span>
    }
}
