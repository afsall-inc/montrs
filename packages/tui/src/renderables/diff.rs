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

/// Diff renderable — unified/split diff display.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMode {
    Unified,
    Split,
}

pub struct DiffRenderable {
    pub lines: Vec<DiffLine>,
    pub mode: DiffMode,
    pub scroll_offset: usize,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub text: String,
    pub kind: DiffLineKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Add,
    Remove,
    Context,
    Header,
}

impl DiffRenderable {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            mode: DiffMode::Unified,
            scroll_offset: 0,
        }
    }
    pub fn parse_unified(mut self, diff_text: &str) -> Self {
        for line in diff_text.lines() {
            let kind = if line.starts_with("+") {
                DiffLineKind::Add
            } else if line.starts_with("-") {
                DiffLineKind::Remove
            } else if line.starts_with("@@") {
                DiffLineKind::Header
            } else {
                DiffLineKind::Context
            };
            self.lines.push(DiffLine {
                text: line.to_string(),
                kind,
            });
        }
        self
    }
}

impl Default for DiffRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for DiffRenderable {
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
            let (fg, bg) = match line.kind {
                DiffLineKind::Add => {
                    (Color::Rgb(80, 200, 80), Color::Rgb(20, 60, 20))
                }
                DiffLineKind::Remove => {
                    (Color::Rgb(200, 80, 80), Color::Rgb(60, 20, 20))
                }
                DiffLineKind::Header => (Color::Cyan, Color::Reset),
                DiffLineKind::Context => (Color::Reset, Color::Reset),
            };
            let max_chars = width.min(line.text.len());
            for (i, c) in line.text.chars().enumerate().take(max_chars) {
                buffer.set(
                    x + i,
                    y + row,
                    Cell::styled(c, fg, bg, CharAttribute::default()),
                );
            }
        }
    }
}
