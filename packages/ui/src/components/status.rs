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
    Status {
        base: "inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium",
        variants: {
            variant: {
                Active: "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400",
                Inactive: "bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400",
                Pending: "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400",
                Error: "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400",
            }
        }
    }
}

#[component]
pub fn Status(
    #[prop(into, optional)] variant: Signal<StatusVariant>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = StatusClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };
    view! {
        <span class=merged data-name="Status">
            <span class="h-1.5 w-1.5 rounded-full bg-current" />
            {label}
        </span>
    }
}
