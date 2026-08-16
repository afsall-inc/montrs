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

//! Proxy server — routes subdomains to local ports using axum.

use axum::{
    Router, body::Body, extract::Request, http::StatusCode, response::Response,
    routing::any,
};
use http_body_util::BodyExt;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

/// A route mapping: `<slug>.localhost` -> `127.0.0.1:<port>`.
#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub slug: String,
    pub target_port: u16,
    pub use_tls: bool,
}

/// Configuration for the proxy server.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// The address to listen on (default: "127.0.0.1:8080").
    pub listen_addr: SocketAddr,
    /// Route entries.
    pub routes: Vec<RouteEntry>,
    /// Fallback target port (None = 404).
    pub fallback: Option<u16>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
            routes: Vec::new(),
            fallback: None,
        }
    }
}

/// The proxy server.
#[derive(Clone)]
pub struct ProxyServer {
    route_map: Arc<HashMap<String, u16>>,
    fallback: Option<u16>,
    config: ProxyConfig,
}

impl ProxyServer {
    /// Create a new proxy server.
    pub fn new(config: ProxyConfig) -> Self {
        let mut route_map = HashMap::new();
        for route in &config.routes {
            route_map.insert(route.slug.clone(), route.target_port);
        }
        Self {
            route_map: Arc::new(route_map),
            fallback: config.fallback,
            config,
        }
    }

    /// Build the axum router.
    fn build_router(&self) -> Router {
        let route_map = self.route_map.clone();
        let fallback = self.fallback;

        Router::new().fallback(any(move |req: Request<Body>| {
            let route_map = route_map.clone();
            let fallback = fallback;
            async move {
                let host = req
                    .headers()
                    .get("host")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();

                // Extract slug: <slug>.localhost
                let slug = host
                    .split(':')
                    .next()
                    .unwrap_or("")
                    .strip_suffix(".localhost")
                    .or_else(|| host.strip_suffix(".local"))
                    .unwrap_or("");

                if let Some(&port) = route_map.get(slug) {
                    proxy_request(req, port).await
                } else if let Some(fallback_port) = fallback {
                    proxy_request(req, fallback_port).await
                } else {
                    Ok(Response::builder()
                        .status(404)
                        .body(Body::from("Not Found"))
                        .unwrap())
                }
            }
        }))
    }

    /// Start the proxy server (blocking).
    pub async fn serve(&self) -> anyhow::Result<()> {
        let addr = self.config.listen_addr;
        let app = self.build_router();
        let listener = TcpListener::bind(addr).await?;
        info!("proxy server listening on {addr}");

        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Resolve a hostname to a target port.
    pub fn resolve(&self, host: &str) -> Option<u16> {
        let host = host.split(':').next().unwrap_or(host);
        let slug = host
            .strip_suffix(".localhost")
            .or_else(|| host.strip_suffix(".local"))
            .unwrap_or(host);
        self.route_map.get(slug).copied().or(self.fallback)
    }
}

/// Forward a request to a target port.
async fn proxy_request(
    req: Request<Body>,
    port: u16,
) -> Result<Response<Body>, StatusCode> {
    let target = format!(
        "http://127.0.0.1:{port}{}",
        req.uri()
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/")
    );

    let method = req.method().clone();
    let headers = req.headers().clone();

    // Collect the body.
    let body_bytes = match req.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let client = reqwest::Client::new();
    let proxy_req = client
        .request(method, &target)
        .headers(headers)
        .body(body_bytes.to_vec());

    match proxy_req.send().await {
        Ok(resp) => {
            let status = resp.status();
            let resp_headers = resp.headers().clone();
            let body = resp.bytes().await.unwrap_or_default();
            let mut builder = Response::builder().status(status);
            for (k, v) in resp_headers {
                if let Some(name) = k
                    && name.as_str() != "transfer-encoding"
                {
                    builder = builder.header(name, v);
                }
            }
            Ok(builder
                .body(Body::from(body))
                .unwrap_or_else(|_| Response::new(Body::from("proxy error"))))
        }
        Err(_) => Ok(Response::builder()
            .status(502)
            .body(Body::from("Bad Gateway"))
            .unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut config = ProxyConfig::default();
        config.routes = vec![RouteEntry {
            slug: "api".to_string(),
            target_port: 3001,
            use_tls: false,
        }];
        let proxy = ProxyServer::new(config);
        assert_eq!(proxy.resolve("api.localhost"), Some(3001));
        assert_eq!(proxy.resolve("api.localhost:8080"), Some(3001));
        assert_eq!(proxy.resolve("other.localhost"), None);
    }
}
