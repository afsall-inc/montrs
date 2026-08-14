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

//! montrs-web: Web platform adapter for MontRS.
//!
//! Implements `PlatformAdapter` from `montrs-platform` for browser/WASM targets.
//! Uses `web-sys` and `wasm-bindgen` for DOM and browser API access.

#[cfg(test)]
pub mod test_helpers;

use montrs_platform::{PlatformAdapter, Target};

/// Web platform adapter for browser/WASM environments.
pub struct WebAdapter {
    target: Target,
}

impl WebAdapter {
    pub fn new() -> Self {
        Self {
            target: Target::Web,
        }
    }

    /// Create an adapter for a specific web target.
    pub fn with_target(target: Target) -> Self {
        debug_assert!(target.is_web(), "WebAdapter requires a web target");
        Self { target }
    }
}

impl Default for WebAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for WebAdapter {
    fn target(&self) -> Target {
        self.target
    }

    fn open_url(&self, url: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().expect("no global window");
            let _ = window.location().assign(url);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = url;
        }
    }

    fn set_title(&self, title: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let document = web_sys::window()
                .and_then(|w| w.document());
            if let Some(doc) = document {
                doc.set_title(title);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = title;
        }
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // Browser window size is controlled by the user, not the app
    }

    fn description(&self) -> &'static str {
        "Web platform (browser WASM)"
    }
}