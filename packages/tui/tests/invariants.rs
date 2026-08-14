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

//! Invariant tests for montrs-tui.

use montrs_platform::PlatformAdapter;
use montrs_tui::{
    adapter::TuiAdapter,
    buffer::*,
    event::*,
    renderables::{
        Renderable, box_renderable::BoxRenderable,
        text_renderable::TextRenderable,
    },
    terminal::*,
};

#[test]
fn test_buffer_create() {
    let buffer = Buffer::new(10, 5);
    assert_eq!(buffer.width, 10);
    assert_eq!(buffer.height, 5);
}

#[test]
fn test_buffer_write_str() {
    let mut buffer = Buffer::new(10, 5);
    buffer.write_str(0, 0, "hello");
    assert_eq!(buffer.cell(0, 0).ch, 'h');
    assert_eq!(buffer.cell(4, 0).ch, 'o');
}

#[test]
fn test_buffer_out_of_bounds() {
    let mut buffer = Buffer::new(5, 5);
    // Writing out of bounds should not panic
    buffer.write_str(100, 100, "out of bounds");
    buffer.set(50, 50, Cell::new('x'));
}

#[test]
fn test_buffer_clear() {
    let mut buffer = Buffer::new(5, 5);
    buffer.write_str(0, 0, "hello");
    buffer.clear();
    assert_eq!(buffer.cell(0, 0).ch, ' ');
}

#[test]
fn test_cell_styling() {
    let cell = Cell::styled(
        'x',
        Color::BrightRed,
        Color::Blue,
        CharAttribute::default(),
    );
    assert_eq!(cell.ch, 'x');
    assert_eq!(cell.fg, Color::BrightRed);
    assert_eq!(cell.bg, Color::Blue);
}

#[test]
fn test_color_ansi() {
    assert_eq!(Color::Red.ansi_code(), "\x1b[31m");
    assert_eq!(Color::BrightGreen.ansi_bg(), "\x1b[102m");
    assert!(Color::Rgb(10, 20, 30).ansi_code().contains("38;2;10;20;30"));
}

#[test]
fn test_buffer_to_ansi() {
    let mut buffer = Buffer::new(3, 1);
    buffer.write_str(0, 0, "abc");
    let ansi = buffer.to_ansi();
    assert!(ansi.contains("abc"));
}

#[test]
fn test_parse_escape_seq_arrow_keys() {
    assert_eq!(
        parse_escape_seq(b"\x1b[A"),
        Some(TermEvent::Key(KeyEvent::Up))
    );
    assert_eq!(
        parse_escape_seq(b"\x1b[B"),
        Some(TermEvent::Key(KeyEvent::Down))
    );
    assert_eq!(
        parse_escape_seq(b"\x1b[C"),
        Some(TermEvent::Key(KeyEvent::Right))
    );
    assert_eq!(
        parse_escape_seq(b"\x1b[D"),
        Some(TermEvent::Key(KeyEvent::Left))
    );
}

#[test]
fn test_parse_escape_seq_enter() {
    assert_eq!(
        parse_escape_seq(b"\r"),
        Some(TermEvent::Key(KeyEvent::Enter))
    );
}

#[test]
fn test_parse_escape_seq_escape() {
    assert_eq!(
        parse_escape_seq(b"\x1b"),
        Some(TermEvent::Key(KeyEvent::Escape))
    );
}

#[test]
fn test_parse_escape_seq_char() {
    assert_eq!(
        parse_escape_seq(b"a"),
        Some(TermEvent::Key(KeyEvent::Char('a')))
    );
}

#[test]
fn test_terminal_color_detection() {
    let support = detect_color_support();
    // Should always return at least Ansi
    assert!(matches!(
        support,
        ColorSupport::Ansi | ColorSupport::Ansi256 | ColorSupport::Rgb
    ));
}

#[test]
fn test_box_renderable() {
    let mut buffer = Buffer::new(10, 5);
    let bx = BoxRenderable::new().with_title("Title");
    bx.render(&mut buffer, 0, 0, 10, 5);
    // Corner should be a box-drawing char
    assert_eq!(buffer.cell(0, 0).ch, '╭');
    assert_eq!(buffer.cell(9, 0).ch, '╮');
    assert_eq!(buffer.cell(0, 4).ch, '╰');
    assert_eq!(buffer.cell(9, 4).ch, '╯');
}

#[test]
fn test_text_renderable() {
    let mut buffer = Buffer::new(10, 2);
    let text = TextRenderable::new("hello");
    text.render(&mut buffer, 0, 0, 10, 2);
    assert_eq!(buffer.cell(0, 0).ch, 'h');
    assert_eq!(buffer.cell(4, 0).ch, 'o');
}

#[test]
fn test_platform_adapter_tui() {
    let adapter = montrs_tui::adapter::TuiAdapter::new();
    assert_eq!(adapter.target(), montrs_platform::Target::Tui);
    assert!(adapter.target().is_tui());
    assert!(!adapter.description().is_empty());
}
