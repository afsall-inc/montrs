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

//! montrs-build-serve: Dev server for MontRS projects.
//!
//! Serves the site root directory and optionally spawns the SSR server.
//! Extracted from `montrs-build` to separate the HTTP serving concern
//! from the build pipeline.

use anyhow::Result;
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::info;

/// Configuration for the dev server.
#[derive(Debug, Clone)]
pub struct ServeConfig {
    /// The address to bind to (e.g., "0.0.0.0:3000").
    pub addr: String,
    /// The root directory to serve files from.
    pub site_root: PathBuf,
    /// The WASM package directory relative to site_root.
    pub pkg_dir: PathBuf,
}

/// Start the static file dev server.
///
/// Serves files from `site_root` and logs the address. This is a
/// lightweight static file server — the SSR server binary is spawned
/// separately by the CLI.
pub async fn serve_static(config: ServeConfig) -> Result<()> {
    let app = Router::new().fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev server listening on {}", config.addr);
    info!("Serving from {}", config.site_root.display());

    axum::serve(listener, app).await?;
    Ok(())
}

/// Start the dev server with a callback for when the server is ready.
pub async fn serve_with_callback<F>(
    config: ServeConfig,
    on_ready: F,
) -> Result<()>
where
    F: FnOnce(),
{
    let app = Router::new().fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev server listening on {}", config.addr);
    on_ready();

    axum::serve(listener, app).await?;
    Ok(())
}

/// Fallback page shown when the very first build has not produced an SSR
/// binary yet. It keeps the dev server (and its live-reload socket) alive and
/// renders build errors in place until a successful build takes over.
const FALLBACK_TEMPLATE: &str = r#"<!doctype html>
<html lang="en" class="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>MontRS — compiling…</title>
<style>
:root { color-scheme: dark; }
body { margin:0; background:#0a0a0a; color:#e5e5e5; font:14px/1.6 ui-sans-serif,system-ui,sans-serif; display:flex; min-height:100vh; align-items:center; justify-content:center; }
.box { max-width:680px; width:100%; padding:32px; text-align:center; }
.spin { width:24px; height:24px; border:2px solid #333; border-top-color:#f97316; border-radius:50%; animation:sp .8s linear infinite; margin:0 auto 16px; }
@keyframes sp { to { transform:rotate(360deg); } }
h1 { font-size:16px; margin:0 0 4px; }
p { color:#a1a1aa; margin:0; }
pre { display:none; margin-top:16px; padding:16px; background:#111; border:1px solid #262626; border-radius:10px; text-align:left; font:12px/1.5 ui-monospace,Menlo,monospace; white-space:pre-wrap; max-height:55vh; overflow:auto; color:#fca5a5; }
.err .spin { display:none; }
.err pre { display:block; }
</style>
</head>
<body>
<div class="box" id="box">
  <div class="spin"></div>
  <h1>MontRS is compiling…</h1>
  <p>The first build is in progress. This page reloads automatically when it finishes.</p>
  <pre id="msg"></pre>
</div>
<script>
(function () {
  var port = __RELOAD_PORT__;
  var box = document.getElementById('box');
  var msg = document.getElementById('msg');
  function connect() {
    var ws = new WebSocket('ws://' + location.hostname + ':' + port);
    ws.onopen = function () { ws.send('{"hello":"montrs-overlay"}'); };
    ws.onmessage = function (e) {
      var data;
      try { data = JSON.parse(e.data); } catch (_) { return; }
      if (data.type === 'build-error' || data.type === 'server-error') {
        box.classList.add('err');
        msg.textContent = (data.message || '').trim();
      } else if (data.type === 'build-ok') {
        box.classList.remove('err');
        msg.textContent = '';
        location.reload();
      }
    };
    ws.onclose = function () { setTimeout(connect, 1000); };
  }
  connect();
})();
</script>
</body>
</html>
"#;

/// The rendered fallback page for a given reload port.
pub fn fallback_page(reload_port: u16) -> String {
    FALLBACK_TEMPLATE.replace("__RELOAD_PORT__", &reload_port.to_string())
}

/// Serve the fallback page (and any existing site assets) until `shutdown`
/// resolves. Used while the first build is failing or still running.
pub async fn serve_fallback_with_shutdown<F>(
    config: ServeConfig,
    reload_port: u16,
    shutdown: F,
) -> Result<()>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    let page = fallback_page(reload_port);
    let app = Router::new()
        .route(
            "/",
            axum::routing::get(move || {
                let page = page.clone();
                async move { axum::response::Html(page) }
            }),
        )
        .fallback_service(ServeDir::new(&config.site_root));

    let listener = tokio::net::TcpListener::bind(&config.addr).await?;
    info!("Dev fallback server listening on {}", config.addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
}
