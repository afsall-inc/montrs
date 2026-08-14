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
use std::cell::RefCell;

/// Reactive hover state for an element.
/// Returns `(on_mouse_enter, on_mouse_leave, is_hovered)`.
pub fn use_hover() -> (
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    ReadSignal<bool>,
) {
    let (hovered, set_hovered) = signal(false);
    let on_enter = move |_: leptos::ev::MouseEvent| set_hovered.set(true);
    let on_leave = move |_: leptos::ev::MouseEvent| set_hovered.set(false);
    (on_enter, on_leave, hovered)
}

/// Reactive press/tap state.
pub fn use_press() -> (
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    ReadSignal<bool>,
) {
    let (pressed, set_pressed) = signal(false);
    let on_down = move |_: leptos::ev::MouseEvent| set_pressed.set(true);
    let on_up = move |_: leptos::ev::MouseEvent| set_pressed.set(false);
    (on_down, on_up, pressed)
}

/// Pan/drag tracking with real delta calculation.
/// Returns `(on_mousedown, on_mousemove, on_mouseup, delta_signal, is_dragging)`.
#[allow(clippy::type_complexity)]
pub fn use_pan() -> (
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    ReadSignal<(f64, f64)>,
    ReadSignal<bool>,
) {
    let (delta, set_delta) = signal((0.0f64, 0.0f64));
    let (dragging, set_dragging) = signal(false);
    let start_pos = std::rc::Rc::new(RefCell::new((0.0f64, 0.0f64)));

    let on_down = {
        let start_pos = start_pos.clone();
        move |e: leptos::ev::MouseEvent| {
            start_pos.replace((e.client_x() as f64, e.client_y() as f64));
            set_delta.set((0.0, 0.0));
            set_dragging.set(true);
        }
    };

    let on_move = {
        let start_pos = start_pos.clone();
        move |e: leptos::ev::MouseEvent| {
            if dragging.get() {
                let start = *start_pos.borrow();
                let dx = e.client_x() as f64 - start.0;
                let dy = e.client_y() as f64 - start.1;
                set_delta.set((dx, dy));
            }
        }
    };

    let on_up = move |_: leptos::ev::MouseEvent| {
        set_dragging.set(false);
    };

    (on_down, on_move, on_up, delta, dragging)
}
