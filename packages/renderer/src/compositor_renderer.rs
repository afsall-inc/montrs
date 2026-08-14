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

//! CompositorRenderer — a Renderer implementation backed by the Compositor.
//!
//! Wraps a `Compositor` + a backend `Renderer` so that the compositor layer
//! stack is flushed to the backend on `finish()`. This is the primary way
//! to use the renderer in desktop and mobile apps.

use crate::*;

/// A Renderer implementation that buffers drawing commands through a
/// Compositor and flushes them to a backend Renderer on finish().
pub struct CompositorRenderer {
    compositor: Compositor,
    backend: Box<dyn Renderer>,
    current_viewport: Option<Viewport>,
}

impl CompositorRenderer {
    pub fn new(backend: Box<dyn Renderer>) -> Self {
        Self {
            compositor: Compositor::new(),
            backend,
            current_viewport: None,
        }
    }

    /// Returns a mutable reference to the compositor for direct layer manipulation.
    pub fn compositor(&mut self) -> &mut Compositor {
        &mut self.compositor
    }

    /// Returns a mutable reference to the backend renderer.
    pub fn backend(&mut self) -> &mut dyn Renderer {
        &mut *self.backend
    }

    /// Begin a new compositing layer with the given alpha.
    pub fn begin_layer(&mut self, alpha: f32) {
        self.compositor.begin_layer(alpha);
    }
}

impl Renderer for CompositorRenderer {
    fn begin(&mut self, viewport: &Viewport) {
        self.current_viewport = Some(*viewport);
        self.compositor = Compositor::new();
    }

    fn fill_quad(&mut self, quad: &Quad, paint: &Paint) {
        self.compositor.push_quad(quad.clone(), paint.clone());
    }

    fn fill_path(&mut self, path: &Path, paint: &Paint) {
        self.compositor.push_path(path.clone(), paint.clone());
    }

    fn stroke_path(&mut self, path: &Path, stroke: &Stroke, paint: &Paint) {
        self.compositor.push_stroke(
            path.clone(),
            stroke.clone(),
            paint.clone(),
        );
    }

    fn draw_glyphs(
        &mut self,
        _pos: Point,
        _glyphs: &[GlyphRun],
        _paint: &Paint,
    ) {
        // Compositor does not yet support text glyphs — will be added with text pipeline
    }

    fn draw_image(&mut self, image: &Image, rect: Rect) {
        self.compositor.push_image(image.clone(), rect);
    }

    fn draw_svg(&mut self, svg: &Svg, rect: Rect) {
        self.compositor.push_svg(svg.clone(), rect);
    }

    fn clip(&mut self, _shape: &Shape) {
        // Compositor clipping is pending implementation
    }

    fn clear_clip(&mut self) {
        // Compositor clipping is pending implementation
    }

    fn push_layer(&mut self, alpha: f32, _transform: &[f32; 6]) {
        self.compositor.begin_layer(alpha);
    }

    fn pop_layer(&mut self) {
        // Layers are flushed on finish()
    }

    fn finish(&mut self) -> Frame {
        if let Some(viewport) = &self.current_viewport {
            self.compositor.render(&mut *self.backend, viewport);
        }
        self.backend.finish()
    }
}
