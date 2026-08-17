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

/// Table renderable — text table with borders and selection.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct TextTableRenderable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub selected_row: Option<usize>,
    pub column_widths: Vec<usize>,
}

impl TextTableRenderable {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            selected_row: None,
            column_widths: Vec::new(),
        }
    }
    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.column_widths = headers.iter().map(|h| h.len().max(8)).collect();
        self.headers = headers;
        self
    }
    pub fn add_row(&mut self, row: Vec<String>) {
        for (i, col) in row.iter().enumerate() {
            if i < self.column_widths.len() {
                self.column_widths[i] = self.column_widths[i].max(col.len());
            }
        }
        self.rows.push(row);
    }
}

impl Default for TextTableRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for TextTableRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
    ) {
        let border = Color::Rgb(100, 100, 100);
        let mut row_y = y;
        // Headers
        if !self.headers.is_empty() {
            for (i, header) in self.headers.iter().enumerate() {
                let col_x =
                    x + self.column_widths[..i].iter().sum::<usize>() + i;
                for (j, c) in header.chars().enumerate() {
                    buffer.set(
                        col_x + j,
                        row_y,
                        Cell::styled(
                            c,
                            Color::Cyan,
                            Color::Reset,
                            CharAttribute {
                                bold: true,
                                ..Default::default()
                            },
                        ),
                    );
                }
            }
            row_y += 1;
            // Header separator
            for cx in x..(x
                + self.column_widths.iter().sum::<usize>()
                + self.column_widths.len().saturating_sub(1))
            {
                buffer.set(
                    cx,
                    row_y,
                    Cell::styled(
                        '─',
                        border,
                        Color::Reset,
                        CharAttribute::default(),
                    ),
                );
            }
            row_y += 1;
        }
        // Rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            let is_selected = self.selected_row == Some(row_idx);
            let bg = if is_selected {
                Color::Rgb(40, 40, 80)
            } else {
                Color::Reset
            };
            if is_selected {
                buffer.set(
                    x - 1,
                    row_y,
                    Cell::styled(
                        '>',
                        Color::Cyan,
                        bg,
                        CharAttribute::default(),
                    ),
                );
            }
            for (i, col) in row.iter().enumerate() {
                let col_x =
                    x + self.column_widths[..i].iter().sum::<usize>() + i;
                for (j, c) in col.chars().enumerate() {
                    buffer.set(
                        col_x + j,
                        row_y,
                        Cell::styled(
                            c,
                            Color::Reset,
                            bg,
                            CharAttribute::default(),
                        ),
                    );
                }
                // Vertical separator
                if i < row.len() - 1 {
                    let sep_x = col_x + self.column_widths[i];
                    buffer.set(
                        sep_x,
                        row_y,
                        Cell::styled('│', border, bg, CharAttribute::default()),
                    );
                }
            }
            row_y += 1;
        }
    }
}
