/// Terminal I/O — raw mode, capability detection, and terminal control.
use std::io::{self, Write};

/// Terminal capabilities.
#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    pub color: ColorSupport,
    pub mouse_support: bool,
    pub bracketed_paste: bool,
    pub unicode: bool,
}

/// Color support level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorSupport {
    None,
    #[default]
    Ansi,
    Ansi256,
    Rgb,
}

/// Terminal abstraction.
pub struct Terminal {
    pub raw_mode: bool,
    pub mouse_enabled: bool,
    pub splash_color: ColorSupport,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            raw_mode: false,
            mouse_enabled: false,
            splash_color: detect_color_support(),
        }
    }
}

pub fn detect_color_support() -> ColorSupport {
    if std::env::var("COLORTERM")
        .map(|v| v.contains("truecolor") || v.contains("24bit"))
        .unwrap_or(false)
    {
        ColorSupport::Rgb
    } else if std::env::var("TERM")
        .map(|v| v.contains("256"))
        .unwrap_or(false)
    {
        ColorSupport::Ansi256
    } else {
        ColorSupport::Ansi
    }
}

impl Terminal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enter raw mode (disable line buffering, echo, signal processing).
    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        self.raw_mode = true;
        // Enable mouse and bracketed paste via ANSI sequences.
        io::stdout().write_all(
            b"\x1b[?1049h\x1b[?25l\x1b[?1000h\x1b[?1002h\x1b[?1003h\x1b[?2004h",
        )?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Leave raw mode.
    pub fn leave_raw_mode(&mut self) -> io::Result<()> {
        self.raw_mode = false;
        io::stdout().write_all(
            b"\x1b[?1049l\x1b[?25h\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?2004l",
        )?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Enable mouse reporting.
    pub fn enable_mouse(&mut self) -> io::Result<()> {
        self.mouse_enabled = true;
        io::stdout().write_all(b"\x1b[?1000h\x1b[?1002h\x1b[?1003h")?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Disable mouse reporting.
    pub fn disable_mouse(&mut self) -> io::Result<()> {
        self.mouse_enabled = false;
        io::stdout().write_all(b"\x1b[?1000l\x1b[?1002l\x1b[?1003l")?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Get the terminal size (columns, rows).
    pub fn size(&self) -> io::Result<(usize, usize)> {
        // ioctl TIOCGWINSZ via libc on unix.
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            let fd = io::stdout().as_raw_fd();
            let mut winsize = libc_winsize::Winsize {
                rows: 0,
                cols: 0,
                xpixel: 0,
                ypixel: 0,
            };
            // SAFETY: winsize is a valid pointer.
            unsafe {
                if libc_winsize::ioctl(
                    fd,
                    libc_winsize::TIOCGWINSZ,
                    &mut winsize,
                ) == 0
                {
                    return Ok((winsize.cols as usize, winsize.rows as usize));
                }
            }
        }
        // Fallback: query via ANSI cursor position.
        Err(io::Error::other("cannot determine terminal size"))
    }

    /// Clear the screen.
    pub fn clear(&self) -> io::Result<()> {
        io::stdout().write_all(b"\x1b[2J\x1b[H")?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Move the cursor to (x, y).
    pub fn move_cursor(&self, x: usize, y: usize) -> io::Result<()> {
        write!(io::stdout(), "\x1b[{};{}H", y + 1, x + 1)?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Set the window title.
    pub fn set_title(&self, title: &str) -> io::Result<()> {
        write!(io::stdout(), "\x1b]0;{}\x07", title)?;
        io::stdout().flush()?;
        Ok(())
    }
}

/// Minimal libc FFI for winsize without pulling in libc.
#[cfg(unix)]
mod libc_winsize {
    #[repr(C)]
    pub struct Winsize {
        pub rows: u16,
        pub cols: u16,
        pub xpixel: u16,
        pub ypixel: u16,
    }

    pub const TIOCGWINSZ: u64 = 0x5413;

    #[link(name = "c")]
    unsafe extern "C" {
        pub fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
}
