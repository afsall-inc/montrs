//! montrs-edge: Edge runtime adapter for MontRS.
//!
//! Provides an `EdgeAdapter` implementing `PlatformAdapter` for edge computing
//! environments like Cloudflare Workers and Deno. Also provides a lightweight
//! request handler compatible with the `fetch` event model used by edge runtimes.

use montrs_core::AppSpec;
use montrs_platform::{PlatformAdapter, Target};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Edge platform adapter.
pub struct EdgeAdapter {
    target: Target,
}

impl EdgeAdapter {
    pub fn new() -> Self {
        Self {
            target: Target::Edge,
        }
    }
}

impl Default for EdgeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for EdgeAdapter {
    fn target(&self) -> Target {
        Target::Edge
    }

    fn open_url(&self, _url: &str) {
        // Edge environments typically don't open URLs
    }

    fn set_title(&self, _title: &str) {
        // No window in edge environments
    }

    fn set_size(&self, _width: u32, _height: u32) {
        // No window in edge environments
    }

    fn description(&self) -> &'static str {
        "Edge computing platform (Cloudflare Workers, Deno, etc.)"
    }
}

/// A lightweight edge request/response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// A lightweight edge response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Edge request handler for MontRS applications.
///
/// Processes incoming `EdgeRequest`s against the `AppSpec` router and
/// returns `EdgeResponse`s. This is the edge equivalent of the SSR server.
pub fn handle_edge_request<C>(
    spec: &AppSpec<C>,
    request: EdgeRequest,
) -> EdgeResponse
where
    C: montrs_core::AppConfig + 'static,
{
    // Render the matched route's view
    let view = spec.router.render_view(&request.path);

    // Serialize the rendered HTML
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>MontRS Edge</title>
</head>
<body>
    <div id="app">{}</div>
</body>
</html>"#,
        "Rendered content" // In production, this would be the SSR output
    );

    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());

    EdgeResponse {
        status: 200,
        headers,
        body: html.into_bytes(),
    }
}

/// Cloudflare Workers fetch handler adapter.
///
/// Converts a Cloudflare `Request` to an `EdgeRequest` and returns
/// a `Response`. This is a stub — full integration requires `worker` crate.
#[cfg(feature = "cloudflare")]
pub mod cloudflare {
    use super::*;

    pub async fn handle_fetch<C>(
        _request: &str,
        spec: &AppSpec<C>,
    ) -> EdgeResponse
    where
        C: montrs_core::AppConfig + 'static,
    {
        // Stub: will be implemented with worker-sys bindings
        handle_edge_request(spec, EdgeRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: HashMap::new(),
            body: None,
        })
    }
}

/// Deno adapter.
///
/// Provides a standard `serve` handler compatible with Deno's HTTP API.
/// This is a stub — full integration requires `deno_core` or similar.
#[cfg(feature = "deno")]
pub mod deno {
    use super::*;

    pub async fn handle_request<C>(
        _request: &str,
        spec: &AppSpec<C>,
    ) -> EdgeResponse
    where
        C: montrs_core::AppConfig + 'static,
    {
        // Stub: will be implemented with Deno FFI bindings
        handle_edge_request(spec, EdgeRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: HashMap::new(),
            body: None,
        })
    }
}