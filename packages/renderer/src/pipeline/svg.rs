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

#[cfg(feature = "svg")]
use std::collections::HashMap;

#[cfg(feature = "svg")]
pub struct SvgEntry {
    pub pixmap: tiny_skia::Pixmap,
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "svg")]
pub struct SvgPipeline {
    entries: HashMap<u64, SvgEntry>,
    next_id: u64,
}

#[cfg(not(feature = "svg"))]
pub struct SvgPipeline;

#[cfg(not(feature = "svg"))]
impl Default for SvgPipeline {
    fn default() -> Self {
        Self
    }
}

#[cfg(feature = "svg")]
impl Default for SvgPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "svg")]
impl SvgPipeline {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn render(&mut self, svg_data: &str, width: u32, height: u32) -> u64 {
        let opt = usvg::Options::default();
        let rtree = match usvg::Tree::from_str(svg_data, &opt) {
            Ok(tree) => tree,
            Err(_) => return 0,
        };

        let mut pixmap = match tiny_skia::Pixmap::new(width, height) {
            Some(p) => p,
            None => return 0,
        };

        resvg::render(
            &rtree,
            tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );

        let id = self.next_id;
        self.next_id += 1;
        self.entries.insert(
            id,
            SvgEntry {
                pixmap,
                width,
                height,
            },
        );
        id
    }

    pub fn get_pixmap(&self, id: u64) -> Option<&tiny_skia::Pixmap> {
        self.entries.get(&id).map(|e| &e.pixmap)
    }
}

#[cfg(not(feature = "svg"))]
impl SvgPipeline {
    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &mut self,
        _svg_data: &str,
        _width: u32,
        _height: u32,
    ) -> u64 {
        0
    }
}
