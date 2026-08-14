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

/// Alert dialog overlay.
#[component]
pub fn AlertDialogOverlay(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in \
             data-[state=closed]:animate-out data-[state=closed]:fade-out-0 \
             data-[state=open]:fade-in-0",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="AlertDialogOverlay" />
    }
}

/// Alert dialog content wrapper.
#[component]
pub fn AlertDialogContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg \
             translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background \
             p-6 shadow-lg duration-200 data-[state=open]:animate-in \
             data-[state=closed]:animate-out data-[state=closed]:fade-out-0 \
             data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 \
             data-[state=open]:zoom-in-95 \
             data-[state=closed]:slide-out-to-left-1/2 \
             data-[state=closed]:slide-out-to-top-[48%] \
             data-[state=open]:slide-in-from-left-1/2 \
             data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="AlertDialogContent" role="alertdialog">
            {children()}
        </div>
    }
}

/// Alert dialog header.
#[component]
pub fn AlertDialogHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex flex-col space-y-2 text-center sm:text-left",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="AlertDialogHeader">
            {children()}
        </div>
    }
}

/// Alert dialog footer.
#[component]
pub fn AlertDialogFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="AlertDialogFooter">
            {children()}
        </div>
    }
}

/// Alert dialog title.
#[component]
pub fn AlertDialogTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-lg font-semibold", class.get());

    view! {
        <h2 class=merged data-name="AlertDialogTitle">
            {children()}
        </h2>
    }
}

/// Alert dialog description.
#[component]
pub fn AlertDialogDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <div class=merged data-name="AlertDialogDescription">
            {children()}
        </div>
    }
}

/// Alert dialog action button.
#[component]
pub fn AlertDialogAction(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "inline-flex items-center justify-center whitespace-nowrap \
             rounded-md text-sm font-medium ring-offset-background \
             transition-colors focus-visible:outline-none \
             focus-visible:ring-2 focus-visible:ring-ring \
             focus-visible:ring-offset-2 disabled:pointer-events-none \
             disabled:opacity-50 bg-primary text-primary-foreground \
             hover:bg-primary/90 h-10 px-4 py-2",
            class.get()
        )
    };

    view! {
        <button type="button" class=merged data-name="AlertDialogAction">
            {children()}
        </button>
    }
}

/// Alert dialog cancel button.
#[component]
pub fn AlertDialogCancel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "inline-flex items-center justify-center whitespace-nowrap \
             rounded-md text-sm font-medium ring-offset-background \
             transition-colors focus-visible:outline-none \
             focus-visible:ring-2 focus-visible:ring-ring \
             focus-visible:ring-offset-2 disabled:pointer-events-none \
             disabled:opacity-50 border border-input bg-background \
             hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2 mt-2 \
             sm:mt-0",
            class.get()
        )
    };

    view! {
        <button type="button" class=merged data-name="AlertDialogCancel">
            {children()}
        </button>
    }
}
