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

/// Range slider component.
///
/// Renders a styled range input slider.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Slider min=0 max=100 value=slider_value />
/// }
/// ```
#[component]
pub fn Slider(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] min: f64,
    #[prop(optional)] max: f64,
    #[prop(optional)] step: f64,
    #[prop(into, optional)] value: RwSignal<f64>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "relative flex w-full touch-none select-none items-center",
            class.get()
        )
    };

    let pct = move || {
        let range = max - min;
        if range > 0.0 {
            ((value.get() - min) / range * 100.0).min(100.0)
        } else {
            0.0
        }
    };

    let on_input = move |ev: leptos::ev::Event| {
        let target = event_target_value(&ev);
        if let Ok(v) = target.parse::<f64>() {
            value.set(v);
        }
    };

    let _track_style = move || format!("left: 0%; right: {}%;", 100.0 - pct());
    let range_style = move || format!("left: 0%; width: {}%;", pct());

    view! {
        <div class=merged data-name="Slider">
            <div class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
                <div
                    class="absolute h-full bg-primary"
                    style=range_style
                    data-name="SliderRange"
                />
            </div>
            <input
                type="range"
                min=min
                max=max
                step=step
                value=move || value.get()
                disabled=disabled
                on:input=on_input
                class="absolute inset-0 h-full w-full cursor-pointer opacity-0"
                data-name="SliderInput"
            />
            <div
                class="absolute h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
                style=move || format!("left: calc({}% - 10px);", pct())
                data-name="SliderThumb"
            />
        </div>
    }
}
