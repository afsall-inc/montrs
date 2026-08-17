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

/// Breadcrumb navigation component.
///
/// Renders a navigation trail with separators between items.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Breadcrumb>
///         <BreadcrumbList>
///             <BreadcrumbItem><BreadcrumbLink href="/">"Home"</BreadcrumbLink></BreadcrumbItem>
///             <BreadcrumbSeparator />
///             <BreadcrumbItem><BreadcrumbLink href="/docs">"Docs"</BreadcrumbLink></BreadcrumbItem>
///         </BreadcrumbList>
///     </Breadcrumb>
/// }
/// ```
#[component]
pub fn Breadcrumb(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <nav aria-label="breadcrumb" class=merged data-name="Breadcrumb">
            {children()}
        </nav>
    }
}

/// Breadcrumb list container.
#[component]
pub fn BreadcrumbList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex flex-wrap items-center gap-1.5 break-words text-sm \
             text-muted-foreground sm:gap-2.5",
            class.get()
        )
    };

    view! {
        <ol class=merged data-name="BreadcrumbList">
            {children()}
        </ol>
    }
}

/// Breadcrumb item.
#[component]
pub fn BreadcrumbItem(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("inline-flex items-center gap-1.5", class.get());

    view! {
        <li class=merged data-name="BreadcrumbItem">
            {children()}
        </li>
    }
}

/// Breadcrumb link.
#[component]
pub fn BreadcrumbLink(
    href: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged =
        move || cn!("transition-colors hover:text-foreground", class.get());

    view! {
        <a href=href class=merged data-name="BreadcrumbLink">
            {children()}
        </a>
    }
}

/// Separator between breadcrumb items.
#[component]
pub fn BreadcrumbSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("[&>svg]:size-3.5", class.get());

    view! {
        <li class=merged data-name="BreadcrumbSeparator" aria-hidden="true">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24" height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m9 18 6-6-6-6" />
            </svg>
        </li>
    }
}

/// Ellipsis for truncated breadcrumb trails.
#[component]
pub fn BreadcrumbEllipsis(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged =
        move || cn!("flex h-9 w-9 items-center justify-center", class.get());

    view! {
        <li class=merged data-name="BreadcrumbEllipsis" aria-hidden="true">
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
            <span class="sr-only">"More"</span>
        </li>
    }
}

/// Current page indicator in breadcrumb.
#[component]
pub fn BreadcrumbPage(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("font-normal text-foreground", class.get());

    view! {
        <span role="link" aria-disabled="true" aria-current="page" class=merged data-name="BreadcrumbPage">
            {children()}
        </span>
    }
}
