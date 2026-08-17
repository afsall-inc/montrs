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

use montrs_platform::{PlatformAdapter, Target};
use thiserror::Error;

/// Desktop platform adapter implementing PlatformAdapter.
///
/// Uses wry (webview) or winit + wgpu (native) depending on feature flags.
pub struct DesktopAdapter;

impl DesktopAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DesktopAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for DesktopAdapter {
    fn target(&self) -> Target {
        Target::Desktop
    }

    fn open_url(&self, url: &str) {
        if let Err(e) = open::that(url) {
            eprintln!("Failed to open URL '{url}': {e}");
        }
    }

    fn set_title(&self, _title: &str) {
        // Title is set at window creation time via run_webview / run_native
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // Size is set at window creation time
    }

    fn description(&self) -> &'static str {
        "Desktop platform (wry webview or winit + wgpu native)"
    }
}

/// Launch a desktop application with the given HTML content.
#[cfg(feature = "webview")]
pub fn run_webview(title: &str, html: &str) -> Result<(), DesktopError> {
    use wry::{
        application::{
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(title).build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?.with_html(html)?.build()?;

    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let wry::application::event::Event::WindowEvent {
            event: wry::application::window::WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    })?;

    Ok(())
}

/// Launch a native desktop application with the montrs-renderer.
#[cfg(feature = "native")]
pub fn run_native(
    title: &str,
    renderer: &mut dyn montrs_renderer::Renderer,
) -> Result<(), DesktopError> {
    use std::sync::Arc;
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new()?;
    let window =
        Arc::new(WindowBuilder::new().with_title(title).build(&event_loop)?);

    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let _ = size;
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let size = window.inner_size();
                let viewport = montrs_renderer::Viewport::new(
                    size.width as f32,
                    size.height as f32,
                    1.0,
                );
                renderer.begin(&viewport);
                renderer.finish();
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum DesktopError {
    #[error("WebView error: {0}")]
    WebView(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Window error: {0}")]
    Window(String),
    #[error("Event loop error: {0}")]
    EventLoop(String),
}

#[cfg(feature = "webview")]
impl From<wry::Error> for DesktopError {
    fn from(e: wry::Error) -> Self {
        DesktopError::WebView(e.into())
    }
}

#[cfg(feature = "native")]
impl From<winit::error::EventLoopError> for DesktopError {
    fn from(e: winit::error::EventLoopError) -> Self {
        DesktopError::EventLoop(e.to_string())
    }
}

#[cfg(feature = "native")]
impl From<winit::error::OsError> for DesktopError {
    fn from(e: winit::error::OsError) -> Self {
        DesktopError::Window(e.to_string())
    }
}
