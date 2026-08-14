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

//! Invariant tests for montrs-renderer.
//!
//! Validates that public types construct, data types have expected defaults,
//! and the Renderer trait is usable.

use montrs_renderer::*;

#[test]
fn test_viewport_construct() {
    let vp = Viewport::new(1920.0, 1080.0, 2.0);
    assert_eq!(vp.rect.width, 1920.0);
    assert_eq!(vp.rect.height, 1080.0);
    assert_eq!(vp.scale, 2.0);
}

#[test]
fn test_rect_construct() {
    let rect = Rect {
        x: 10.0,
        y: 20.0,
        width: 100.0,
        height: 200.0,
    };
    assert_eq!(rect.x, 10.0);
    assert_eq!(rect.width, 100.0);
}

#[test]
fn test_point_construct() {
    let p = Point { x: 1.0, y: 2.0 };
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
}

#[test]
fn test_color_constants() {
    assert_eq!(Color::BLACK.r, 0.0);
    assert_eq!(Color::BLACK.a, 1.0);
    assert_eq!(Color::WHITE.r, 1.0);
    assert_eq!(Color::WHITE.g, 1.0);
    assert_eq!(Color::WHITE.b, 1.0);
    assert_eq!(Color::TRANSPARENT.a, 0.0);
}

#[test]
fn test_color_from_rgba8() {
    let color = Color::from_rgba8(255, 0, 0, 255);
    assert!((color.r - 1.0).abs() < 0.001);
    assert!((color.g - 0.0).abs() < 0.001);
    assert!((color.b - 0.0).abs() < 0.001);
    assert!((color.a - 1.0).abs() < 0.001);
}

#[test]
fn test_paint_default() {
    let paint = Paint::default();
    assert_eq!(paint.color.r, 0.0);
    assert!(paint.anti_alias);
}

#[test]
fn test_stroke_default() {
    let stroke = Stroke::default();
    assert_eq!(stroke.width, 1.0);
}

#[test]
fn test_path_construct() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.line_to(100.0, 0.0);
    path.line_to(100.0, 100.0);
    path.close();
    assert_eq!(path.segments.len(), 4);
}

#[test]
fn test_quad_construct() {
    let quad = Quad {
        rect: Rect {
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 50.0,
        },
        corner_radius: 5.0,
    };
    assert_eq!(quad.corner_radius, 5.0);
}

#[test]
fn test_frame_construct() {
    let frame = Frame {
        data: vec![0u8; 100],
        width: 10,
        height: 10,
    };
    assert_eq!(frame.data.len(), 100);
}

#[test]
fn test_renderer_trait_object_safe() {
    struct MockRenderer;
    impl Renderer for MockRenderer {
        fn begin(&mut self, _vp: &Viewport) {}
        fn fill_quad(&mut self, _q: &Quad, _p: &Paint) {}
        fn fill_path(&mut self, _p: &Path, _pt: &Paint) {}
        fn stroke_path(&mut self, _p: &Path, _s: &Stroke, _pt: &Paint) {}
        fn draw_glyphs(&mut self, _pos: Point, _g: &[GlyphRun], _p: &Paint) {}
        fn draw_image(&mut self, _img: &Image, _r: Rect) {}
        fn draw_svg(&mut self, _svg: &Svg, _r: Rect) {}
        fn clip(&mut self, _s: &Shape) {}
        fn clear_clip(&mut self) {}
        fn push_layer(&mut self, _a: f32, _t: &[f32; 6]) {}
        fn pop_layer(&mut self) {}
        fn finish(&mut self) -> Frame {
            Frame {
                data: vec![],
                width: 0,
                height: 0,
            }
        }
    }
    let mut renderer: Box<dyn Renderer> = Box::new(MockRenderer);
    let vp = Viewport::new(100.0, 100.0, 1.0);
    renderer.begin(&vp);
    let frame = renderer.finish();
    assert_eq!(frame.width, 0);
}
