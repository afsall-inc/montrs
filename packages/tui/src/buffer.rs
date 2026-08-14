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

/// Terminal buffer system — cells, colors, and rendering buffers.
use std::fmt;

/// A color in the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl Color {
    pub fn ansi_code(&self) -> String {
        match self {
            Color::Reset => "\x1b[39m".to_string(),
            Color::Black => "\x1b[30m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::BrightBlack => "\x1b[90m".to_string(),
            Color::BrightRed => "\x1b[91m".to_string(),
            Color::BrightGreen => "\x1b[92m".to_string(),
            Color::BrightYellow => "\x1b[93m".to_string(),
            Color::BrightBlue => "\x1b[94m".to_string(),
            Color::BrightMagenta => "\x1b[95m".to_string(),
            Color::BrightCyan => "\x1b[96m".to_string(),
            Color::BrightWhite => "\x1b[97m".to_string(),
            Color::Rgb(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
            Color::Indexed(n) => format!("\x1b[38;5;{n}m"),
        }
    }

    pub fn ansi_bg(&self) -> String {
        match self {
            Color::Reset => "\x1b[49m".to_string(),
            Color::Black => "\x1b[40m".to_string(),
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::BrightBlack => "\x1b[100m".to_string(),
            Color::BrightRed => "\x1b[101m".to_string(),
            Color::BrightGreen => "\x1b[102m".to_string(),
            Color::BrightYellow => "\x1b[103m".to_string(),
            Color::BrightBlue => "\x1b[104m".to_string(),
            Color::BrightMagenta => "\x1b[105m".to_string(),
            Color::BrightCyan => "\x1b[106m".to_string(),
            Color::BrightWhite => "\x1b[107m".to_string(),
            Color::Rgb(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
            Color::Indexed(n) => format!("\x1b[48;5;{n}m"),
        }
    }
}

/// Text attributes for a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CharAttribute {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub reverse: bool,
}

impl CharAttribute {
    pub const fn new_static() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            dim: false,
            reverse: false,
        }
    }

    pub fn ansi_prefix(&self) -> String {
        let mut codes = Vec::new();
        if self.bold {
            codes.push("1");
        }
        if self.italic {
            codes.push("3");
        }
        if self.underline {
            codes.push("4");
        }
        if self.strikethrough {
            codes.push("9");
        }
        if self.dim {
            codes.push("2");
        }
        if self.reverse {
            codes.push("7");
        }
        if codes.is_empty() {
            return String::new();
        }
        format!("\x1b[{}m", codes.join(";"))
    }

    pub fn ansi_reset(&self) -> String {
        if self.bold
            || self.italic
            || self.underline
            || self.strikethrough
            || self.dim
            || self.reverse
        {
            "\x1b[0m".to_string()
        } else {
            String::new()
        }
    }
}

/// A single character cell in the buffer.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attr: CharAttribute,
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::Reset,
            bg: Color::Reset,
            attr: CharAttribute::default(),
        }
    }

    pub const fn new_static(ch: char) -> Self {
        Self {
            ch,
            fg: Color::Reset,
            bg: Color::Reset,
            attr: CharAttribute::new_static(),
        }
    }

    pub fn styled(ch: char, fg: Color, bg: Color, attr: CharAttribute) -> Self {
        Self { ch, fg, bg, attr }
    }

    pub fn is_empty(&self) -> bool {
        self.ch == ' '
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(' ')
    }
}

/// A 2D buffer of cells.
#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        if x >= self.width || y >= self.height {
            static EMPTY: Cell = Cell::new_static(' ');
            return &EMPTY;
        }
        &self.cells[y * self.width + x]
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        if x >= self.width || y >= self.height {
            panic!(
                "cell out of bounds: ({x}, {y}) in {}x{}",
                self.width, self.height
            );
        }
        &mut self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    pub fn write_str(&mut self, x: usize, y: usize, s: &str) {
        let mut cx = x;
        for ch in s.chars() {
            if cx >= self.width {
                break;
            }
            self.set(cx, y, Cell::new(ch));
            cx += 1;
        }
    }

    pub fn write_str_styled(
        &mut self,
        x: usize,
        y: usize,
        s: &str,
        fg: Color,
        bg: Color,
        attr: CharAttribute,
    ) {
        let mut cx = x;
        for ch in s.chars() {
            if cx >= self.width {
                break;
            }
            self.set(cx, y, Cell::styled(ch, fg, bg, attr));
            cx += 1;
        }
    }

    pub fn fill_rect(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        cell: Cell,
    ) {
        for row in 0..h {
            for col in 0..w {
                self.set(x + col, y + row, cell.clone());
            }
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    /// Render the buffer to an ANSI string.
    pub fn to_ansi(&self) -> String {
        let mut out = String::new();
        let mut last_fg = Color::Reset;
        let mut last_bg = Color::Reset;
        let mut last_attr = CharAttribute::default();

        for y in 0..self.height {
            if y > 0 {
                out.push('\n');
            }
            for x in 0..self.width {
                let cell = self.cell(x, y);
                if cell.fg != last_fg {
                    out.push_str(&cell.fg.ansi_code());
                    last_fg = cell.fg;
                }
                if cell.bg != last_bg {
                    out.push_str(&cell.bg.ansi_bg());
                    last_bg = cell.bg;
                }
                if cell.attr != last_attr {
                    out.push_str(&cell.attr.ansi_prefix());
                    last_attr = cell.attr;
                }
                out.push(cell.ch);
            }
        }
        out
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}
