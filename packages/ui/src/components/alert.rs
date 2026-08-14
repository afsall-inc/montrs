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

crate::variants! {
    Alert {
        base: "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground",
        variants: {
            variant: {
                Default: "bg-background text-foreground",
                Destructive: "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive",
            }
        }
    }
}

/// Alert component for displaying messages.
///
/// Renders an alert with optional title, description, and icon.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Alert variant=AlertVariant::Destructive>
///         <AlertTitle>"Error"</AlertTitle>
///         <AlertDescription>"Something went wrong."</AlertDescription>
///     </Alert>
/// }
/// ```
#[component]
pub fn Alert(
    #[prop(into, optional)] variant: Signal<AlertVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = AlertClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };

    view! {
        <div role="alert" class=merged data-name="Alert">
            {children()}
        </div>
    }
}

/// Alert title.
#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!("mb-1 font-medium leading-none tracking-tight", class.get())
    };

    view! {
        <h5 class=merged data-name="AlertTitle">
            {children()}
        </h5>
    }
}

/// Alert description.
#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm [&_p]:leading-relaxed", class.get());

    view! {
        <div class=merged data-name="AlertDescription">
            {children()}
        </div>
    }
}
