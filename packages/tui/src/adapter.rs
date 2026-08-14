/// TuiAdapter — implements PlatformAdapter for Target::Tui.
use montrs_platform::{PlatformAdapter, Target};

/// Platform adapter for TUI (terminal) targets.
pub struct TuiAdapter;

impl TuiAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TuiAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for TuiAdapter {
    fn target(&self) -> Target {
        Target::Tui
    }

    fn open_url(&self, url: &str) {
        // Open URL via terminal detection — try xdg-open, open, etc.
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }

    fn set_title(&self, title: &str) {
        // Use OSC 0 to set terminal title.
        print!("\x1b]0;{}\x07", title);
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // Terminal size is controlled by the terminal emulator, not the app.
    }

    fn description(&self) -> &'static str {
        "Terminal UI application"
    }
}
