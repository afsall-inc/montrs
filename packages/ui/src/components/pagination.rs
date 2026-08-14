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

/// Pagination component with page numbers.
///
/// Renders a paginated navigation control.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Pagination>
///         <PaginationContent>
///             <PaginationItem>
///                 <PaginationPrevious href="#" />
///             </PaginationItem>
///             <PaginationItem>
///                 <PaginationLink href="#" is_active=true>1</PaginationLink>
///             </PaginationItem>
///             <PaginationItem>
///                 <PaginationNext href="#" />
///             </PaginationItem>
///         </PaginationContent>
///     </Pagination>
/// }
/// ```
#[component]
pub fn Pagination(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("mx-auto flex w-full justify-center", class.get());

    view! {
        <nav role="navigation" aria-label="pagination" class=merged data-name="Pagination">
            {children()}
        </nav>
    }
}

/// Pagination content list.
#[component]
pub fn PaginationContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-row items-center gap-1", class.get());

    view! {
        <ul class=merged data-name="PaginationContent">
            {children()}
        </ul>
    }
}

/// Pagination item.
#[component]
pub fn PaginationItem(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <li class=merged data-name="PaginationItem">
            {children()}
        </li>
    }
}

/// Pagination link button.
#[component]
pub fn PaginationLink(
    href: String,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] is_active: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "inline-flex items-center justify-center whitespace-nowrap \
                    rounded-md text-sm font-medium ring-offset-background \
                    transition-colors focus-visible:outline-none \
                    focus-visible:ring-2 focus-visible:ring-ring \
                    focus-visible:ring-offset-2 disabled:pointer-events-none \
                    disabled:opacity-50";
        let active = if is_active {
            "border border-input bg-background hover:bg-accent \
             hover:text-accent-foreground h-10 w-10"
        } else {
            "hover:bg-accent hover:text-accent-foreground h-10 w-10"
        };
        cn!(base, active, class.get())
    };

    let aria_current = is_active.then_some("page");

    view! {
        <a href=href class=merged aria-current=aria_current data-name="PaginationLink">
            {children()}
        </a>
    }
}

/// Previous page button.
#[component]
pub fn PaginationPrevious(
    href: String,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "inline-flex items-center justify-center whitespace-nowrap \
             rounded-md text-sm font-medium ring-offset-background \
             transition-colors focus-visible:outline-none \
             focus-visible:ring-2 focus-visible:ring-ring \
             focus-visible:ring-offset-2 disabled:pointer-events-none \
             disabled:opacity-50 hover:bg-accent hover:text-accent-foreground \
             h-10 px-4 py-2 gap-1 pl-2.5",
            class.get()
        )
    };

    view! {
        <a href=href class=merged data-name="PaginationPrevious" aria-label="Go to previous page">
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
                <path d="m15 18-6-6 6-6" />
            </svg>
            <span>"Previous"</span>
        </a>
    }
}

/// Next page button.
#[component]
pub fn PaginationNext(
    href: String,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "inline-flex items-center justify-center whitespace-nowrap \
             rounded-md text-sm font-medium ring-offset-background \
             transition-colors focus-visible:outline-none \
             focus-visible:ring-2 focus-visible:ring-ring \
             focus-visible:ring-offset-2 disabled:pointer-events-none \
             disabled:opacity-50 hover:bg-accent hover:text-accent-foreground \
             h-10 px-4 py-2 gap-1 pr-2.5",
            class.get()
        )
    };

    view! {
        <a href=href class=merged data-name="PaginationNext" aria-label="Go to next page">
            <span>"Next"</span>
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
                <path d="m9 18 6-6-6-6" />
            </svg>
        </a>
    }
}

/// Pagination ellipsis.
#[component]
pub fn PaginationEllipsis(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged =
        move || cn!("flex h-9 w-9 items-center justify-center", class.get());

    view! {
        <li class=merged data-name="PaginationEllipsis" aria-hidden="true">
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
                <circle cx="12" cy="12" r="1" />
                <circle cx="19" cy="12" r="1" />
                <circle cx="5" cy="12" r="1" />
            </svg>
            <span class="sr-only">"More pages"</span>
        </li>
    }
}
