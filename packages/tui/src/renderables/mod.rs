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

/// Renderable components for the TUI.
pub mod ascii_font;
pub mod box_renderable;
pub mod code;
pub mod diff;
pub mod editor;
pub mod frame_buffer;
pub mod image;
pub mod input;
pub mod line_number;
pub mod markdown;
pub mod scrollbar;
pub mod scrollbox;
pub mod select;
pub mod slider;
pub mod tab_select;
pub mod table;
pub mod text_buffer;
pub mod text_renderable;
pub mod textarea;

use crate::buffer::Buffer;
pub use ascii_font::{ASCIIFont, ASCIIFontRenderable};
pub use box_renderable::{BorderStyle, BoxRenderable};
pub use code::CodeRenderable;
pub use diff::{DiffLine, DiffLineKind, DiffMode, DiffRenderable};
pub use editor::EditorRenderable;
pub use frame_buffer::FrameBufferRenderable;
pub use image::{ImageMode, ImageRenderable};
pub use input::InputRenderable;
pub use line_number::LineNumberRenderable;
pub use markdown::MarkdownRenderable;
pub use scrollbar::ScrollBarRenderable;
pub use scrollbox::ScrollBoxRenderable;
pub use select::SelectRenderable;
pub use slider::SliderRenderable;
pub use tab_select::TabSelectRenderable;
pub use table::TextTableRenderable;
pub use text_buffer::TextBufferRenderable;
pub use text_renderable::TextRenderable;
pub use textarea::TextareaRenderable;

/// Base trait for all renderable objects.
pub trait Renderable: Send + Sync {
    /// Render this object into a buffer at the given position and size.
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    );
    /// The minimum width needed.
    fn min_width(&self) -> usize {
        1
    }
    /// The minimum height needed.
    fn min_height(&self) -> usize {
        1
    }
}
