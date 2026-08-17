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
