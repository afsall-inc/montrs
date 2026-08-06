//! montrs-web: Web platform adapter for MontRS.
//!
//! Implements `PlatformAdapter` from `montrs-platform` for browser/WASM targets.
//! Uses `web-sys` and `wasm-bindgen` for DOM and browser API access.

#[cfg(test)]
pub mod test_helpers;

use montrs_platform::{PlatformAdapter, Target};

/// Web platform adapter for browser/WASM environments.
pub struct WebAdapter {
    target: Target,
}

impl WebAdapter {
    pub fn new() -> Self {
        Self {
            target: Target::Wasm,
        }
    }

    /// Create an adapter for a specific web target.
    pub fn with_target(target: Target) -> Self {
        debug_assert!(target.is_web(), "WebAdapter requires a web target");
        Self { target }
    }
}

impl Default for WebAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for WebAdapter {
    fn target(&self) -> Target {
        self.target
    }

    fn open_url(&self, url: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().expect("no global window");
            let _ = window.location().assign(url);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = url;
        }
    }

    fn set_title(&self, title: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let document = web_sys::window()
                .and_then(|w| w.document());
            if let Some(doc) = document {
                doc.set_title(title);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = title;
        }
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // Browser window size is controlled by the user, not the app
    }

    fn description(&self) -> &'static str {
        "Web platform (browser WASM)"
    }
}