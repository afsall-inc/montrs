// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::types::{HapticsProvider, ImpactStyle};

pub struct WebHapticsProvider;

impl WebHapticsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl HapticsProvider for WebHapticsProvider {
    fn vibrate(&self, duration_ms: u32) {
        if let Some(nav) = web_sys::window().and_then(|w| w.navigator()) {
            let _ = nav.vibrate_with_duration(duration_ms);
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        if let Some(nav) = web_sys::window().and_then(|w| w.navigator()) {
            let js_arr = wasm_bindgen::JsValue::from(
                pattern.iter().map(|d| *d as f64).collect::<Vec<f64>>(),
            );
            let _ = nav.vibrate(&js_arr);
        }
    }

    fn impact(&self, style: ImpactStyle) {
        let ms = match style {
            ImpactStyle::Light => 10,
            ImpactStyle::Medium => 20,
            ImpactStyle::Heavy => 40,
            ImpactStyle::Rigid => 30,
            ImpactStyle::Soft => 15,
        };
        self.vibrate(ms);
    }

    fn selection_changed(&self) {
        self.vibrate(5);
    }

    fn is_supported(&self) -> bool {
        web_sys::window()
            .and_then(|w| w.navigator())
            .map(|n| n.vibrate(0).is_ok())
            .unwrap_or(false)
    }

    fn description(&self) -> &str {
        "Web Vibration API via navigator.vibrate()"
    }
}

