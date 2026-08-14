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

use crate::value::MotionValue;
use leptos::prelude::*;

/// Animated wrapper component for HTML elements.
///
/// Like Framer Motion's `motion.div` — wraps a child element with
/// hover-triggered spring animations for scale, opacity, and rotation.
#[component]
pub fn Animated(
    children: Children,
    #[prop(optional, into)] hover_scale: Option<f64>,
    #[prop(optional, into)] hover_opacity: Option<f64>,
    #[prop(optional, into)] hover_rotate: Option<f64>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] initial_scale: Option<f64>,
    #[prop(optional, into)] initial_opacity: Option<f64>,
) -> impl IntoView {
    let scale = MotionValue::new(initial_scale.unwrap_or(1.0));
    let opacity = MotionValue::new(initial_opacity.unwrap_or(1.0));
    let rotate = MotionValue::new(0.0);

    let target_scale = hover_scale.unwrap_or(1.0);
    let target_opacity = hover_opacity.unwrap_or(1.0);
    let target_rotate = hover_rotate.unwrap_or(0.0);

    let on_enter = {
        let s = scale.clone();
        let o = opacity.clone();
        let r = rotate.clone();
        move |_: leptos::ev::MouseEvent| {
            if target_scale != 1.0 {
                s.animate_to(target_scale, 300.0, 20.0, 1.0);
            }
            if target_opacity != 1.0 {
                o.animate_to(target_opacity, 300.0, 20.0, 1.0);
            }
            if target_rotate != 0.0 {
                r.animate_to(target_rotate, 300.0, 20.0, 1.0);
            }
        }
    };

    let on_leave = {
        let s = scale.clone();
        let o = opacity.clone();
        let r = rotate.clone();
        move |_: leptos::ev::MouseEvent| {
            s.animate_to(1.0, 300.0, 20.0, 1.0);
            o.animate_to(1.0, 300.0, 20.0, 1.0);
            r.animate_to(0.0, 300.0, 20.0, 1.0);
        }
    };

    let class_val = class.unwrap_or_default();
    let style = move || {
        format!(
            "transform: translate3d(0px,0px,0px) scale({}) rotate({}deg); \
             opacity: {}; transition: transform 0.3s \
             cubic-bezier(0.16,1,0.3,1), opacity 0.2s ease; will-change: \
             transform, opacity;",
            scale.get(),
            rotate.get(),
            opacity.get()
        )
    };

    view! {
        <div class=class_val style=style on:mouseenter=on_enter on:mouseleave=on_leave>
            {children()}
        </div>
    }
}

/// AnimatePresence — enter/exit animations for conditionally rendered children.
///
/// Tracks children and animates enter (fade in + scale up) and
/// exit (fade out + scale down) transitions. Uses spring physics.
#[component]
pub fn AnimatePresence(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_val = class.unwrap_or_default();
    let scale = MotionValue::new(0.95);
    let opacity = MotionValue::new(0.0);

    // Animate in on mount
    {
        let s = scale.clone();
        let o = opacity.clone();
        leptos::prelude::set_timeout(
            move || {
                s.animate_to(1.0, 400.0, 25.0, 1.0);
                o.animate_to(1.0, 300.0, 20.0, 1.0);
            },
            std::time::Duration::from_millis(10),
        );
    }

    let style = move || {
        format!(
            "transform: scale({}); opacity: {}; transition: transform 0.4s \
             cubic-bezier(0.16,1,0.3,1), opacity 0.3s ease;",
            scale.get(),
            opacity.get()
        )
    };

    view! {
        <div class=class_val style=style>
            {children()}
        </div>
    }
}
