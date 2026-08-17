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

/// Card component with optional header, content, and footer.
///
/// A versatile container for content with consistent styling.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Card>
///         <CardHeader>
///             <CardTitle>"Title"</CardTitle>
///             <CardDescription>"Description"</CardDescription>
///         </CardHeader>
///         <CardContent>"Content"</CardContent>
///         <CardFooter>"Footer"</CardFooter>
///     </Card>
/// }
/// ```
#[component]
pub fn Card(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "rounded-lg border bg-card text-card-foreground shadow-sm",
            class.get()
        )
    };

    view! {
        <div class=merged data-name="Card">
            {children()}
        </div>
    }
}

/// Card header section.
#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col space-y-1.5 p-6", class.get());

    view! {
        <div class=merged data-name="CardHeader">
            {children()}
        </div>
    }
}

/// Card title.
#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "text-2xl font-semibold leading-none tracking-tight",
            class.get()
        )
    };

    view! {
        <h3 class=merged data-name="CardTitle">
            {children()}
        </h3>
    }
}

/// Card description.
#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <p class=merged data-name="CardDescription">
            {children()}
        </p>
    }
}

/// Card content area.
#[component]
pub fn CardContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("p-6 pt-0", class.get());

    view! {
        <div class=merged data-name="CardContent">
            {children()}
        </div>
    }
}

/// Card footer section.
#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center p-6 pt-0", class.get());

    view! {
        <div class=merged data-name="CardFooter">
            {children()}
        </div>
    }
}
