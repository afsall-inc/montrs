//! montrs-mobile: Mobile shell for MontRS applications.
//!
//! Supports Android and iOS via WebView (wry).

/// Launch a mobile application with the given HTML content.
#[cfg(feature = "webview")]
pub fn run_webview(title: &str, html: &str) -> Result<(), MobileError> {
    // TODO: Implement wry-based mobile webview for Android/iOS
    let _ = (title, html);
    Ok(())
}

/// Launch a native mobile application with the montrs-renderer.
#[cfg(feature = "native")]
pub fn run_native(_title: &str) -> Result<(), MobileError> {
    Ok(())
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MobileError {
    #[error("Mobile error: {0}")]
    Generic(String),
}