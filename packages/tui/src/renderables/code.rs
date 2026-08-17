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

/// Code renderable — syntax-highlighted code display.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct CodeRenderable {
    pub lines: Vec<String>,
    pub language: Option<String>,
    pub scroll_offset: usize,
}

impl CodeRenderable {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            language: None,
            scroll_offset: 0,
        }
    }
    pub fn with_code(mut self, code: &str) -> Self {
        self.lines = code.lines().map(|s| s.to_string()).collect();
        self
    }
    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_string());
        self
    }
}

impl Default for CodeRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for CodeRenderable {
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
                let fg = simple_highlight(c, &self.language);
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, Color::Reset, CharAttribute::default()),
                );
            }
        }
    }
}

fn simple_highlight(c: char, _lang: &Option<String>) -> Color {
    match c {
        '#' | ';' | '/' => Color::Rgb(120, 120, 120), // comments
        '"' | '\'' => Color::Rgb(210, 180, 140),      // strings
        '0'..='9' => Color::Rgb(220, 220, 100),       // numbers
        _ => Color::Reset,
    }
}
