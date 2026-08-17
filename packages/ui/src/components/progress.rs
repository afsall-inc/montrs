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

/// Progress bar component.
///
/// Renders a horizontal progress indicator.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Progress value=75 max=100 />
/// }
/// ```
#[component]
pub fn Progress(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] value: f64,
    #[prop(optional)] max: f64,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "relative h-4 w-full overflow-hidden rounded-full bg-secondary",
            class.get()
        )
    };

    let pct = if max > 0.0 {
        (value / max * 100.0).min(100.0)
    } else {
        0.0
    };
    let indicator_style = format!("transform: translateX(-{}%)", 100.0 - pct);

    view! {
        <div
            class=merged
            role="progressbar"
            aria-valuenow=value as i64
            aria-valuemin=0
            aria-valuemax=max as i64
            data-name="Progress"
        >
            <div
                class="h-full w-full flex-1 bg-primary transition-all duration-300 ease-in-out"
                style=indicator_style
                data-name="ProgressIndicator"
            />
        </div>
    }
}
