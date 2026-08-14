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

/// LineNumber renderable — line number gutter.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct LineNumberRenderable {
    pub line_count: usize,
    pub scroll_offset: usize,
    pub active_line: Option<usize>,
}

impl LineNumberRenderable {
    pub fn new() -> Self {
        Self {
            line_count: 0,
            scroll_offset: 0,
            active_line: None,
        }
    }
    pub fn with_line_count(mut self, count: usize) -> Self {
        self.line_count = count;
        self
    }
}

impl Default for LineNumberRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for LineNumberRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        let width = width.clamp(3, 6);
        for row in 0..height {
            let line_num = self.scroll_offset + row + 1;
            if line_num > self.line_count {
                break;
            }
            let is_active = self.active_line == Some(line_num - 1);
            let fg = if is_active {
                Color::Cyan
            } else {
                Color::Rgb(100, 100, 100)
            };
            let num_str = format!("{:>width$}", line_num, width = width);
            for (i, c) in num_str.chars().enumerate() {
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, Color::Reset, CharAttribute::default()),
                );
            }
        }
    }
}
