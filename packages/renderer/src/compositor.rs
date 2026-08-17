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

use crate::{Paint, Path, Quad, Rect, Renderer, Stroke, Viewport};

pub struct Layer {
    pub quads: Vec<Quad>,
    pub paints: Vec<Paint>,
    pub paths: Vec<(Path, Paint)>,
    pub strokes: Vec<(Path, Stroke, Paint)>,
    pub images: Vec<(crate::Image, Rect)>,
    pub svgs: Vec<(crate::Svg, Rect)>,
    pub alpha: f32,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            quads: Vec::new(),
            paints: Vec::new(),
            paths: Vec::new(),
            strokes: Vec::new(),
            images: Vec::new(),
            svgs: Vec::new(),
            alpha: 1.0,
        }
    }
}

pub struct Compositor {
    layers: Vec<Layer>,
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

impl Compositor {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn begin_layer(&mut self, alpha: f32) {
        self.layers.push(Layer {
            alpha,
            ..Default::default()
        });
    }

    pub fn push_quad(&mut self, quad: Quad, paint: Paint) {
        if let Some(layer) = self.layers.last_mut() {
            layer.quads.push(quad);
            layer.paints.push(paint);
        }
    }

    pub fn push_path(&mut self, path: Path, paint: Paint) {
        if let Some(layer) = self.layers.last_mut() {
            layer.paths.push((path, paint));
        }
    }

    pub fn push_stroke(&mut self, path: Path, stroke: Stroke, paint: Paint) {
        if let Some(layer) = self.layers.last_mut() {
            layer.strokes.push((path, stroke, paint));
        }
    }

    pub fn push_image(&mut self, image: crate::Image, rect: Rect) {
        if let Some(layer) = self.layers.last_mut() {
            layer.images.push((image, rect));
        }
    }

    pub fn push_svg(&mut self, svg: crate::Svg, rect: Rect) {
        if let Some(layer) = self.layers.last_mut() {
            layer.svgs.push((svg, rect));
        }
    }

    pub fn render(&mut self, renderer: &mut dyn Renderer, viewport: &Viewport) {
        renderer.begin(viewport);
        for layer in &self.layers {
            renderer.push_layer(layer.alpha, &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

            for (quad, paint) in layer.quads.iter().zip(layer.paints.iter()) {
                renderer.fill_quad(quad, paint);
            }
            for (path, paint) in &layer.paths {
                renderer.fill_path(path, paint);
            }
            for (path, stroke, paint) in &layer.strokes {
                renderer.stroke_path(path, stroke, paint);
            }
            for (image, rect) in &layer.images {
                renderer.draw_image(image, *rect);
            }
            for (svg, rect) in &layer.svgs {
                renderer.draw_svg(svg, *rect);
            }

            renderer.pop_layer();
        }
        renderer.finish();
    }
}
