use montrs_platform::{PlatformAdapter, Target};
use thiserror::Error;

/// Mobile platform adapter implementing PlatformAdapter.
///
/// For now, this is a stub that provides the adapter interface. Real
/// Android/iOS native bridges will be added in future releases.
pub struct MobileAdapter {
    target: Target,
}

impl MobileAdapter {
    pub fn new(target: Target) -> Self {
        debug_assert!(target.is_mobile(), "MobileAdapter requires a mobile target");
        Self { target }
    }
}

impl PlatformAdapter for MobileAdapter {
    fn target(&self) -> Target {
        self.target
    }

    fn open_url(&self, _url: &str) {
        // Stub — will use Android Intents / iOS UIApplication
    }

    fn set_title(&self, _title: &str) {
        // Stub — will set native window title
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // Stub — mobile sizes are fixed by device
    }

    fn description(&self) -> &'static str {
        match self.target {
            Target::MobileAndroid => "Android mobile platform",
            Target::MobileIos => "iOS mobile platform",
            _ => "Mobile platform (unknown target)",
        }
    }
}

/// Launch a mobile application with the given HTML content.
#[cfg(feature = "webview")]
pub fn run_webview(title: &str, html: &str) -> Result<(), MobileError> {
    let event_loop = wry::application::event_loop::EventLoop::new();
    let window = wry::application::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)?;
    let _webview = wry::webview::WebViewBuilder::new(window)?
        .with_html(html)?
        .build()?;

    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = wry::application::event_loop::ControlFlow::Wait;
        if let wry::application::event::Event::WindowEvent {
            event: wry::application::window::WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = wry::application::event_loop::ControlFlow::Exit;
        }
    })?;

    Ok(())
}

/// Launch a native mobile application with the montrs-renderer.
#[cfg(feature = "native")]
pub fn run_native(title: &str) -> Result<(), MobileError> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let _window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)?;

    event_loop.run(move |_event, _window_target, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
    })?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum MobileError {
    #[error("Mobile error: {0}")]
    Generic(String),
    #[error("WebView error: {0}")]
    WebView(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Event loop error: {0}")]
    EventLoop(String),
}

#[cfg(feature = "webview")]
impl From<wry::Error> for MobileError {
    fn from(e: wry::Error) -> Self {
        MobileError::WebView(e.into())
    }
}

#[cfg(feature = "native")]
impl From<winit::error::EventLoopError> for MobileError {
    fn from(e: winit::error::EventLoopError) -> Self {
        MobileError::EventLoop(e.to_string())
    }
}

#[cfg(feature = "native")]
impl From<winit::error::OsError> for MobileError {
    fn from(e: winit::error::OsError) -> Self {
        MobileError::Generic(e.to_string())
    }
}