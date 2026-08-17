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

/// Slider renderable — scrollbar/slider indicator.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct SliderRenderable {
    pub position: f64, // 0.0 to 1.0
    pub length: f64,   // visible fraction (0.0 to 1.0)
}

impl SliderRenderable {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            length: 0.1,
        }
    }
    pub fn with_range(
        mut self,
        pos: usize,
        visible: usize,
        total: usize,
    ) -> Self {
        if total > 0 {
            self.position = pos as f64 / total as f64;
            self.length = (visible as f64 / total as f64).min(1.0);
        }
        self
    }
}

impl Default for SliderRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for SliderRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        _height: usize,
    ) {
        if width < 2 {
            return;
        }
        let thumb_start =
            (self.position * (width as f64 - 1.0)).round() as usize;
        let thumb_end = ((self.position + self.length) * (width as f64 - 1.0))
            .round() as usize;
        let thumb_end = thumb_end.min(width - 1);
        for i in 0..width {
            let ch = if i >= thumb_start && i <= thumb_end {
                '█'
            } else {
                '░'
            };
            let fg = if i >= thumb_start && i <= thumb_end {
                Color::Cyan
            } else {
                Color::Rgb(80, 80, 80)
            };
            buffer.set(
                x + i,
                y,
                Cell::styled(ch, fg, Color::Reset, CharAttribute::default()),
            );
        }
    }
}
