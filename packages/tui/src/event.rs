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

/// Event system — keyboard, mouse, and terminal events.
use std::sync::mpsc;
use std::thread;

/// Key event types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyEvent {
    Char(char),
    Enter,
    Tab,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    Function(u8),
    Ctrl(char),
    Alt(char),
    Shift(char),
    Unknown,
}

/// Mouse event types.
#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Press {
        button: MouseButton,
        x: usize,
        y: usize,
    },
    Release {
        x: usize,
        y: usize,
    },
    Drag {
        x: usize,
        y: usize,
    },
    Scroll {
        dx: i32,
        dy: i32,
    },
}

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    None,
}

/// A terminal event.
#[derive(Debug, Clone, PartialEq)]
pub enum TermEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Resize { width: usize, height: usize },
    FocusGained,
    FocusLost,
}

/// Parse an ANSI escape sequence into a TermEvent.
pub fn parse_escape_seq(buf: &[u8]) -> Option<TermEvent> {
    if buf.is_empty() {
        return None;
    }

    // Simple parser for common sequences.
    match buf[0] {
        b'\x1b' => {
            if buf.len() == 1 {
                return Some(TermEvent::Key(KeyEvent::Escape));
            }
            match buf.get(1) {
                Some(b'[') => {
                    // CSI sequence.
                    let rest = std::str::from_utf8(&buf[2..]).ok()?;
                    match rest {
                        "A" => return Some(TermEvent::Key(KeyEvent::Up)),
                        "B" => return Some(TermEvent::Key(KeyEvent::Down)),
                        "C" => return Some(TermEvent::Key(KeyEvent::Right)),
                        "D" => return Some(TermEvent::Key(KeyEvent::Left)),
                        "H" => return Some(TermEvent::Key(KeyEvent::Home)),
                        "F" => return Some(TermEvent::Key(KeyEvent::End)),
                        "2~" => return Some(TermEvent::Key(KeyEvent::Insert)),
                        "3~" => return Some(TermEvent::Key(KeyEvent::Delete)),
                        "5~" => return Some(TermEvent::Key(KeyEvent::PageUp)),
                        "6~" => {
                            return Some(TermEvent::Key(KeyEvent::PageDown));
                        }
                        _ => {
                            // Try function keys.
                            if let Some(c) = rest.strip_prefix('1')
                                && let Ok(n) = c
                                    .strip_suffix('~')
                                    .unwrap_or("")
                                    .parse::<u8>()
                            {
                                return Some(TermEvent::Key(
                                    KeyEvent::Function(n),
                                ));
                            }
                            // Try mouse events.
                            if rest.starts_with('M') || rest.starts_with('<') {
                                return Some(TermEvent::Mouse(
                                    MouseEvent::Press {
                                        button: MouseButton::None,
                                        x: 0,
                                        y: 0,
                                    },
                                ));
                            }
                            return Some(TermEvent::Key(KeyEvent::Unknown));
                        }
                    }
                }
                Some(b'O') => {
                    // SS3 sequence.
                    let rest = std::str::from_utf8(&buf[2..]).ok()?;
                    match rest {
                        "P" => {
                            return Some(TermEvent::Key(KeyEvent::Function(1)));
                        }
                        "Q" => {
                            return Some(TermEvent::Key(KeyEvent::Function(2)));
                        }
                        "R" => {
                            return Some(TermEvent::Key(KeyEvent::Function(3)));
                        }
                        "S" => {
                            return Some(TermEvent::Key(KeyEvent::Function(4)));
                        }
                        _ => {}
                    }
                }
                _ => {
                    // Alt+key.
                    if let Some(&c) = buf.get(1)
                        && c != b'['
                        && c != b'O'
                    {
                        return Some(TermEvent::Key(KeyEvent::Alt(c as char)));
                    }
                }
            }
            None
        }
        b'\r' | b'\n' => Some(TermEvent::Key(KeyEvent::Enter)),
        b'\t' => Some(TermEvent::Key(KeyEvent::Tab)),
        0x7f => Some(TermEvent::Key(KeyEvent::Backspace)),
        n if (1..=26).contains(&n) => {
            Some(TermEvent::Key(KeyEvent::Ctrl((n + 96) as char)))
        }
        n => Some(TermEvent::Key(KeyEvent::Char(n as char))),
    }
}

/// An event bus that receives terminal events.
pub struct EventBus {
    rx: mpsc::Receiver<TermEvent>,
    tx: mpsc::Sender<TermEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { rx, tx }
    }

    pub fn sender(&self) -> mpsc::Sender<TermEvent> {
        self.tx.clone()
    }

    pub fn recv(&self) -> Result<TermEvent, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn try_recv(&self) -> Result<TermEvent, mpsc::TryRecvError> {
        self.rx.try_recv()
    }
}

/// An event emitter that runs a background thread reading stdin.
pub struct EventEmitter {
    handle: Option<thread::JoinHandle<()>>,
    tx: mpsc::Sender<TermEvent>,
}

impl EventEmitter {
    pub fn new(tx: mpsc::Sender<TermEvent>) -> Self {
        Self { handle: None, tx }
    }

    pub fn start(&mut self) {
        let tx = self.tx.clone();
        self.handle = Some(thread::spawn(move || {
            use std::io::Read;
            let mut stdin = std::io::stdin();
            let mut buf = vec![0u8; 1024];
            loop {
                match stdin.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let data = &buf[..n];
                        if data.len() == 1 && data[0] == b'\x1b' {
                            let _ = tx.send(TermEvent::Key(KeyEvent::Escape));
                        } else if let Some(event) = parse_escape_seq(data) {
                            let _ = tx.send(event);
                        } else {
                            // Treat as typed characters.
                            for &b in data {
                                let _ = tx.send(TermEvent::Key(
                                    KeyEvent::Char(b as char),
                                ));
                            }
                        }
                    }
                }
            }
        }));
    }
}
