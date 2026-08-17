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

/// Scrollbox renderable — scrollable viewport with content.
use crate::buffer::{Buffer, Cell};
use crate::renderables::Renderable;

pub struct ScrollBoxRenderable {
    pub content: Vec<String>,
    pub scroll_x: usize,
    pub scroll_y: usize,
}

impl ScrollBoxRenderable {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            scroll_x: 0,
            scroll_y: 0,
        }
    }
    pub fn with_content(mut self, content: Vec<String>) -> Self {
        self.content = content;
        self
    }
}

impl Default for ScrollBoxRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ScrollBoxRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        for row in 0..height {
            let line_idx = self.scroll_y + row;
            if line_idx >= self.content.len() {
                break;
            }
            let line = &self.content[line_idx];
            let start = self.scroll_x.min(line.len());
            let end = (start + width).min(line.len());
            for (i, c) in line[start..end].chars().enumerate() {
                buffer.set(x + i, y + row, Cell::new(c));
            }
        }
    }
}
