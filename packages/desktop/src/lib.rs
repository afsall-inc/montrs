//! montrs-desktop: Desktop shell for MontRS applications.
//!
//! Provides two modes:
//! - **WebView** (default): Embeds the MontRS web app in a native window via `wry`
//! - **Native** (opt-in): Renders with `montrs-renderer` + `winit` for GPU-accelerated native UI

/// Launch a desktop application with the given HTML content.
#[cfg(feature = "webview")]
pub fn run_webview(title: &str, html: &str) -> Result<(), DesktopError> {
    use wry::application::{
        event_loop::{EventLoop, ControlFlow},
        window::WindowBuilder,
    };
    use wry::webview::WebViewBuilder;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_html(html)?
        .build()?;

    event_loop.run(|_event, _window_target, control_flow| {
        *control_flow = ControlFlow::Exit;
    });

    Ok(())
}

/// Launch a native desktop application with the montrs-renderer.
#[cfg(feature = "native")]
pub fn run_native(title: &str, _renderer: &mut dyn montrs_renderer::Renderer) -> Result<(), DesktopError> {
    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)?;

    // TODO: Wire up event loop with renderer
    Ok(())
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DesktopError {
    #[error("WebView error: {0}")]
    WebView(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Window error: {0}")]
    Window(String),
}

#[cfg(feature = "webview")]
impl From<wry::Error> for DesktopError {
    fn from(e: wry::Error) -> Self {
        DesktopError::WebView(e.into())
    }
}