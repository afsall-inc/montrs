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

/// Textarea renderable — multi-line text input.
use crate::buffer::{Buffer, Cell};
use crate::renderables::Renderable;

pub struct TextareaRenderable {
    pub lines: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
}

impl TextareaRenderable {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }
    pub fn with_value(mut self, val: &str) -> Self {
        self.lines = val.lines().map(|s| s.to_string()).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self
    }
    pub fn insert(&mut self, c: char) {
        if self.cursor_line >= self.lines.len() {
            return;
        }
        self.lines[self.cursor_line].insert(self.cursor_col, c);
        self.cursor_col += 1;
    }
    pub fn newline(&mut self) {
        let line = self.lines[self.cursor_line].clone();
        let (left, right) = line.split_at(self.cursor_col);
        self.lines[self.cursor_line] = left.to_string();
        self.lines.insert(self.cursor_line + 1, right.to_string());
        self.cursor_line += 1;
        self.cursor_col = 0;
    }
    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.lines[self.cursor_line].remove(self.cursor_col - 1);
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            let prev_len = self.lines[self.cursor_line - 1].len();
            let cur = self.lines.remove(self.cursor_line);
            self.lines[self.cursor_line - 1].push_str(&cur);
            self.cursor_line -= 1;
            self.cursor_col = prev_len;
        }
    }
    pub fn value(&self) -> String {
        self.lines.join("\n")
    }
}

impl Default for TextareaRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TextareaRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        for row in 0..height {
            let line_idx = self.scroll_offset + row;
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let max_chars = width.min(line.len());
            for (i, c) in line.chars().enumerate().take(max_chars) {
                buffer.set(x + i, y + row, Cell::new(c));
            }
        }
    }
}
