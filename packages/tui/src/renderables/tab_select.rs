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

/// TabSelect renderable — horizontal tab bar.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct TabSelectRenderable {
    pub tabs: Vec<String>,
    pub selected: usize,
}

impl TabSelectRenderable {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selected: 0,
        }
    }
    pub fn with_tabs(mut self, tabs: Vec<String>) -> Self {
        self.tabs = tabs;
        self
    }
    pub fn select(&mut self, idx: usize) {
        self.selected = idx.min(self.tabs.len().saturating_sub(1));
    }
    pub fn next(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = (self.selected + 1) % self.tabs.len();
        }
    }
    pub fn prev(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = self.selected.saturating_sub(1);
        }
    }
}

impl Default for TabSelectRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TabSelectRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
    ) {
        let mut cx = x;
        for (idx, tab) in self.tabs.iter().enumerate() {
            let is_selected = idx == self.selected;
            let text = format!(" {} ", tab);
            let fg = if is_selected {
                Color::Black
            } else {
                Color::Reset
            };
            let bg = if is_selected {
                Color::Cyan
            } else {
                Color::Reset
            };
            for (i, c) in text.chars().enumerate() {
                buffer.set(
                    cx + i,
                    y,
                    Cell::styled(c, fg, bg, CharAttribute::default()),
                );
            }
            cx += text.len();
            // Separator between tabs
            if idx < self.tabs.len() - 1 {
                buffer.set(
                    cx,
                    y,
                    Cell::styled(
                        '│',
                        Color::Rgb(100, 100, 100),
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
                cx += 1;
            }
        }
    }
}
