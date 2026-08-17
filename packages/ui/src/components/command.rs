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

/// Command palette / search component (Cmd+K style).
///
/// Renders a command dialog with search input and grouped items.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Command>
///         <CommandInput placeholder="Type a command..." />
///         <CommandList>
///             <CommandGroup heading="Suggestions">
///                 <CommandItem>"Settings"</CommandItem>
///             </CommandGroup>
///         </CommandList>
///     </Command>
/// }
/// ```
#[component]
pub fn Command(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex h-full w-full flex-col overflow-hidden rounded-md \
             bg-popover text-popover-foreground",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="Command">
            {children()}
        </div>
    }
}

/// Command search input.
#[component]
pub fn CommandInput(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex h-11 w-full rounded-md bg-transparent py-3 text-sm \
             outline-none placeholder:text-muted-foreground \
             disabled:cursor-not-allowed disabled:opacity-50",
            class.get()
        )
    };

    view! {
        <div class="flex items-center border-b px-3" cmdk-input-wrapper="">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24" height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="mr-2 h-4 w-4 shrink-0 opacity-50"
            >
                <circle cx="11" cy="11" r="8" />
                <path d="m21 21-4.3-4.3" />
            </svg>
            <input
                class=merged
                placeholder=placeholder
                data-name="CommandInput"
            />
        </div>
    }
}

/// Command list container.
#[component]
pub fn CommandList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "max-h-[300px] overflow-y-auto overflow-x-hidden",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="CommandList" role="listbox">
            {children()}
        </div>
    }
}

/// Empty state for command.
#[component]
pub fn CommandEmpty(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("py-6 text-center text-sm", class.get());

    view! {
        <div class=merged data-name="CommandEmpty">
            {children()}
        </div>
    }
}

/// Command group with heading.
#[component]
pub fn CommandGroup(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] heading: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "overflow-hidden p-1 text-foreground \
             [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:py-1.5 \
             [&_[cmdk-group-heading]]:text-xs \
             [&_[cmdk-group-heading]]:font-medium \
             [&_[cmdk-group-heading]]:text-muted-foreground",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="CommandGroup" role="group">
            {heading.map(|h| view! {
                <div cmdk-group-heading="">{h}</div>
            })}
            {children()}
        </div>
    }
}

/// Command item (selectable).
#[component]
pub fn CommandItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "relative flex cursor-default select-none items-center rounded-sm \
             px-2 py-1.5 text-sm outline-none aria-selected:bg-accent \
             aria-selected:text-accent-foreground \
             data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
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
            role="option"
            data-name="CommandItem"
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

/// Command separator.
#[component]
pub fn CommandSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="CommandSeparator" />
    }
}

/// Command shortcut badge.
#[component]
pub fn CommandShortcut(
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
        <span class=merged data-name="CommandShortcut">
            {children()}
        </span>
    }
}
