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

/// CSS transition and animation helpers.
///
/// Provides utility functions for generating CSS transition properties,
/// will-change hints, and transform strings — all GPU-accelerated
/// through compositor-driven properties.
/// Build a CSS `transition` property value.
pub fn css_transition(
    properties: &[&str],
    duration: &str,
    easing: &str,
) -> String {
    properties
        .iter()
        .map(|p| format!("{} {} {}", p, duration, easing))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Build a CSS `will-change` property value for GPU acceleration hints.
pub fn will_change(properties: &[&str]) -> String {
    properties.join(", ")
}

/// Preset: fast opacity transition.
pub const FADE: &str = "opacity 0.15s ease-in-out";

/// Preset: smooth transform + opacity transition.
pub const SLIDE_UP: &str =
    "transform 0.3s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s ease";

/// Preset: scale transform transition.
pub const SCALE: &str = "transform 0.2s cubic-bezier(0.16, 1, 0.3, 1)";

/// Preset: spring-like transform transition.
pub const SPRING: &str = "transform 0.5s cubic-bezier(0.16, 1, 0.3, 1)";

/// Generate a CSS transform string for 3D GPU acceleration.
pub fn gpu_transform(
    translate_x: f64,
    translate_y: f64,
    scale: f64,
    rotate: f64,
) -> String {
    format!(
        "translate3d({}px, {}px, 0px) scale({}) rotate({}deg)",
        translate_x, translate_y, scale, rotate
    )
}

/// Generate a CSS transform string for 2D transforms (no GPU hint).
pub fn transform_2d(x: f64, y: f64, scale: f64, rotate: f64) -> String {
    format!(
        "translate({}px, {}px) scale({}) rotate({}deg)",
        x, y, scale, rotate
    )
}
