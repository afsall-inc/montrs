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
    /// Server-side rendering (SSR) context.
    Server,
    /// Client-side WASM hydration or CSR context.
    Wasm,
    /// Edge computing environments (e.g., Cloudflare Workers).
    Edge,
    /// Desktop applications (e.g., via Tauri or wry).
    Desktop,
    /// Android mobile platform.
    MobileAndroid,
    /// iOS mobile platform.
    MobileIos,
}

impl Target {
    /// Returns true if the target is a mobile platform.
    pub fn is_mobile(self) -> bool {
        matches!(self, Self::MobileAndroid | Self::MobileIos)
    }

    /// Returns true if the target is a desktop platform.
    pub fn is_desktop(self) -> bool {
        matches!(self, Self::Desktop)
    }

    /// Returns true if the target is a web platform (server or client).
    pub fn is_web(self) -> bool {
        matches!(self, Self::Server | Self::Wasm | Self::Edge)
    }

    /// Human-readable description of the target.
    pub fn description(self) -> &'static str {
        match self {
            Self::Server => "Server-side rendering",
            Self::Wasm => "Client-side WASM hydration",
            Self::Edge => "Edge computing environment",
            Self::Desktop => "Desktop application",
            Self::MobileAndroid => "Android mobile application",
            Self::MobileIos => "iOS mobile application",
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