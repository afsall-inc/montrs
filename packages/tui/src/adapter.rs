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

/// TuiAdapter — implements PlatformAdapter for Target::Tui.
use montrs_platform::{PlatformAdapter, Target};

/// Platform adapter for TUI (terminal) targets.
pub struct TuiAdapter;

impl TuiAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TuiAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for TuiAdapter {
    fn target(&self) -> Target {
        Target::Tui
    }

    fn open_url(&self, url: &str) {
        // Open URL via terminal detection — try xdg-open, open, etc.
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }

    fn set_title(&self, title: &str) {
        // Use OSC 0 to set terminal title.
        print!("\x1b]0;{}\x07", title);
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // Terminal size is controlled by the terminal emulator, not the app.
    }

    fn description(&self) -> &'static str {
        "Terminal UI application"
    }
}
