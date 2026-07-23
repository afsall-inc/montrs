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

#[cfg(target_os = "windows")]
extern "system" {
    fn MessageBeep(uType: u32) -> i32;
}

pub struct DesktopHapticsProvider;

impl Default for DesktopHapticsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopHapticsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl HapticsProvider for DesktopHapticsProvider {
    fn vibrate(&self, _duration_ms: u32) {
        // TODO(#61): wire to montrs-desktop engine once available
        #[cfg(target_os = "windows")]
        unsafe {
            // Temporary: WinAPI MessageBeep for basic audible/tactile feedback
            MessageBeep(0xFFFFFFFF);
        }
        #[cfg(target_os = "macos")]
        {
            // Placeholder: will use NSHapticFeedbackManager via montrs-desktop engine
            // See montrs-desktop for the proper integration
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            // Linux/other: no universal haptics API without the desktop engine
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        // TODO(#61): wire to montrs-desktop engine once available
        for &ms in pattern {
            self.vibrate(ms);
        }
    }

    fn impact(&self, style: ImpactStyle) {
        // TODO(#61): wire to montrs-desktop engine once available
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
        // TODO(#61): wire to montrs-desktop engine once available
        self.vibrate(5);
    }

    fn is_supported(&self) -> bool {
        cfg!(target_os = "macos") || cfg!(target_os = "windows")
    }

    fn description(&self) -> &str {
        "Desktop haptics via OS-native APIs (temporary until montrs-desktop \
         engine)"
    }
}

