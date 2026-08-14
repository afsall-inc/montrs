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

use leptos::prelude::*;

crate::variants! {
    Callout {
        base: "relative w-full rounded-lg border-l-4 p-4",
        variants: {
            variant: {
                Default: "border-l-primary bg-muted/50 text-foreground",
                Info: "border-l-blue-500 bg-blue-50/50 text-foreground dark:bg-blue-950/20",
                Warning: "border-l-yellow-500 bg-yellow-50/50 text-foreground dark:bg-yellow-950/20",
                Error: "border-l-destructive bg-destructive/10 text-foreground",
                Success: "border-l-green-500 bg-green-50/50 text-foreground dark:bg-green-950/20",
            }
        }
    }
}

#[component]
pub fn Callout(
    #[prop(into, optional)] variant: Signal<CalloutVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = CalloutClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };
    view! {
        <div class=merged data-name="Callout">
            {children()}
        </div>
    }
}
