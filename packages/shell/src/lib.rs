//! Shell integration — activation scripts, shell hooks, and shim management.
//!
//! Provides shell-specific activation scripts that set up PATH,
//! environment variables, and shell hooks for MontRS-managed tools.

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

mod bash;
mod fish;
mod pwsh;
pub mod shims;
mod zsh;

/// Supported shell types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Pwsh,
}

impl ShellType {
    pub fn as_shell(&self) -> Box<dyn Shell> {
        match self {
            Self::Bash => Box::<bash::Bash>::default(),
            Self::Zsh => Box::<zsh::Zsh>::default(),
            Self::Fish => Box::<fish::Fish>::default(),
            Self::Pwsh => Box::<pwsh::Pwsh>::default(),
        }
    }

    /// Detect the shell from the SHELL environment variable.
    pub fn detect() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_default();
        let name = std::path::Path::new(&shell)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        match name {
            "zsh" => Self::Zsh,
            "fish" => Self::Fish,
            "pwsh" | "powershell" => Self::Pwsh,
            _ => Self::Bash,
        }
    }
}

impl Display for ShellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bash => write!(f, "bash"),
            Self::Zsh => write!(f, "zsh"),
            Self::Fish => write!(f, "fish"),
            Self::Pwsh => write!(f, "pwsh"),
        }
    }
}

impl FromStr for ShellType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "fish" => Ok(Self::Fish),
            "pwsh" | "powershell" => Ok(Self::Pwsh),
            _ => Err(format!("unknown shell: {s}")),
        }
    }
}

/// Options passed to activation script generation.
#[derive(Debug, Clone, Default)]
pub struct ActivateOptions {
    pub exe: std::path::PathBuf,
    pub flags: String,
    pub no_hook: bool,
}

/// The shell trait — generate shell-specific activation scripts.
pub trait Shell: Display {
    /// Generate the activation script (eval'd into shell rc file).
    fn activate(&self, opts: &ActivateOptions) -> String;

    /// Generate the deactivation script.
    fn deactivate(&self) -> String;

    /// Set an environment variable.
    fn set_env(&self, key: &str, val: &str) -> String;

    /// Unset an environment variable.
    fn unset_env(&self, key: &str) -> String;

    /// Prepend a directory to PATH.
    fn prepend_path(&self, dir: &str) -> String;

    /// Generate a shell hook (for prompt or chpwd).
    fn hook_prompt(&self) -> String {
        String::new()
    }

    /// Generate the hook-env script (runs every prompt/enter to sync env).
    fn hook_env(&self, _opts: &ActivateOptions) -> String {
        format!(
            "export PATH=\"{}\":$PATH\n",
            montrs_tool::backend::default_shims_dir().display()
        )
    }
}
