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

//! montrs-platform: Platform abstraction layer for MontRS.
//!
//! Provides the `Target` enum (moved from `montrs-core`), the `PlatformAdapter`
//! trait, and platform-specific implementations for native desktop, mobile,
//! and web shells. This crate is layer-0: it has zero MontRS-internal dependencies.

pub mod native_menu;

use serde::{Deserialize, Serialize};

/// The execution environment target for the application.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Target {
    /// Unified web target — can be SSR or static export. Deployment mode is
    /// determined at build time by the `montrs.toml [deploy]` section.
    Web,
    /// Desktop applications (e.g., via wry or winit).
    Desktop,
    /// Mobile applications (Android + iOS, PlatformAdapter handles OS).
    Mobile,
    /// Terminal UI applications.
    Tui,
}

impl Target {
    /// Returns true if the target is a mobile platform.
    pub fn is_mobile(self) -> bool {
        matches!(self, Self::Mobile)
    }

    /// Returns true if the target is a desktop platform.
    pub fn is_desktop(self) -> bool {
        matches!(self, Self::Desktop)
    }

    /// Returns true if the target is a web platform.
    pub fn is_web(self) -> bool {
        matches!(self, Self::Web)
    }

    /// Returns true if the target is a TUI platform.
    pub fn is_tui(self) -> bool {
        matches!(self, Self::Tui)
    }

    /// Human-readable description of the target.
    pub fn description(self) -> &'static str {
        match self {
            Self::Web => "Web application (SSR or static export)",
            Self::Desktop => "Desktop application",
            Self::Mobile => "Mobile application",
            Self::Tui => "Terminal UI application",
        }
    }
}

/// A platform adapter provides target-specific capabilities to the framework.
///
/// Each platform (web, desktop, mobile) implements this trait so that the
/// rest of MontRS can interact with native features without conditional
/// compilation scattered across the codebase.
pub trait PlatformAdapter: Send + Sync {
    /// Returns the target this adapter represents.
    fn target(&self) -> Target;

    /// Open a URL in the default browser (or platform equivalent).
    fn open_url(&self, url: &str);

    /// Set the window title. No-op on platforms without a window.
    fn set_title(&self, title: &str);

    /// Set the window size. No-op on platforms without a window.
    fn set_size(&self, width: u32, height: u32);

    /// Returns a human-readable description of this adapter.
    fn description(&self) -> &'static str;
}

/// A no-op platform adapter for environments where no native platform is
/// available (e.g., pure server context).
pub struct NoopPlatformAdapter {
    target: Target,
}

impl NoopPlatformAdapter {
    pub fn new(target: Target) -> Self {
        Self { target }
    }
}

impl PlatformAdapter for NoopPlatformAdapter {
    fn target(&self) -> Target {
        self.target
    }

    fn open_url(&self, _url: &str) {}

    fn set_title(&self, _title: &str) {}

    fn set_size(&self, _width: u32, _height: u32) {}

    fn description(&self) -> &'static str {
        "No-op platform adapter"
    }
}
