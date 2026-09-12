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

//! montrs-core/src/serve.rs: SSR server entry point.
//!
//! Provides `montrs_serve` — a single function call that replaces the ~40 lines
//! of boilerplate in every app's `main.rs`. Creates its own single-threaded
//! tokio runtime with LocalSet for Leptos SSR compatibility.

#[cfg(feature = "ssr")]
use crate::{AppConfig, Router};

/// Start an Axum SSR server backed by a MontRS Router.
///
/// Creates a single-threaded tokio runtime with a `LocalSet` to support
/// Leptos `spawn_local` during SSR rendering. Reads `MONTRS_SITE_ADDR`,
/// `MONTRS_SITE_ROOT`, and `MONTRS_SITE_PKG_DIR` from the environment.
///
/// # Example
/// ```rust,ignore
/// #[cfg(feature = "ssr")]
/// fn main() {
///     tracing_subscriber::fmt().with_env_filter("info").init();
///     let spec = app::build_spec();
///     montrs_core::serve::montrs_serve(spec.router, || view! { <app::App /> })
///         .unwrap();
/// }
/// ```
#[cfg(feature = "ssr")]
pub fn montrs_serve<C, F, IV>(
    router: Router<C>,
    app_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    C: AppConfig + 'static,
    F: Fn() -> IV + Clone + Send + Sync + 'static,
    IV: leptos::prelude::IntoView + 'static,
{
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async move { serve_inner(router, app_fn).await })
}

#[cfg(feature = "ssr")]
async fn serve_inner<C, F, IV>(
    router: Router<C>,
    app_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    C: AppConfig + 'static,
    F: Fn() -> IV + Clone + Send + Sync + 'static,
    IV: leptos::prelude::IntoView + 'static,
{
    use axum::Router as AxumRouter;
    use leptos::prelude::*;
    use leptos_axum::LeptosRoutes;
    use tokio::task::LocalSet;
    use tower_http::services::ServeDir;

    // MontRS is the single source of truth for site config. Derive Leptos
    // runtime env vars from MONTRS_* values (set by the CLI from montrs.toml)
    // so the runtime no longer depends on `[package.metadata.leptos]`.
    let addr = std::env::var("MONTRS_SITE_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let site_root = std::env::var("MONTRS_SITE_ROOT")
        .unwrap_or_else(|_| "target/site".to_string());
    let pkg_dir = std::env::var("MONTRS_SITE_PKG_DIR")
        .unwrap_or_else(|_| "pkg".to_string());
    let output_name = std::env::var("MONTRS_OUTPUT_NAME")
        .unwrap_or_else(|_| "website".to_string());
    let reload_port = std::env::var("MONTRS_RELOAD_PORT")
        .unwrap_or_else(|_| "3001".to_string());

    unsafe {
        std::env::set_var("LEPTOS_OUTPUT_NAME", &output_name);
        std::env::set_var("LEPTOS_SITE_ADDR", &addr);
        std::env::set_var("LEPTOS_SITE_ROOT", &site_root);
        std::env::set_var("LEPTOS_SITE_PKG_DIR", &pkg_dir);
        std::env::set_var("LEPTOS_RELOAD_PORT", &reload_port);
        // Leptos only injects the live-reload script (which opens the
        // WebSocket to the reload port) when LEPTOS_WATCH is set.
        std::env::set_var("LEPTOS_WATCH", "1");
    }

    let mut conf = get_configuration(None).unwrap();

    // The pkg dir must stay a site-root-relative URL path (e.g. "pkg").
    // An absolute filesystem path here (which the CLI could pass through)
    // leaks `\\?\C:\...` into the hydration bootstrap's `import()` specifier
    // and breaks WASM loading entirely.
    let relative_pkg = std::path::Path::new(&*conf.leptos_options.site_pkg_dir)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| pkg_dir.clone());
    conf.leptos_options.site_pkg_dir = relative_pkg.into();

    let axum_routes = router.to_axum_route_listings();

    let mut app = AxumRouter::new()
        .leptos_routes_with_context(
            &conf.leptos_options,
            axum_routes,
            {
                let r = router.clone();
                let leptos_options = conf.leptos_options.clone();
                move || {
                    provide_context(r.clone());
                    // The SSR shell reads `LeptosOptions` (output_name,
                    // site_root, pkg dir) to render the hydration bootstrap
                    // scripts that match the WASM bundle names.
                    provide_context(leptos_options.clone());
                }
            },
            app_fn,
        )
        .fallback_service(ServeDir::new(&site_root));

    // Next.js-style dev overlay, injected by the framework itself (not the
    // app) so it appears in every MontRS app during `montrs serve`/`watch`.
    if std::env::var("LEPTOS_WATCH").is_ok() {
        app = app.layer(axum::middleware::from_fn(inject_dev_overlay));
    }

    let app = app
        // Dev servers must never serve stale bundles: the hydration entry
        // (`/pkg/front.js`, `/pkg/front_bg.wasm`) and stylesheets use fixed
        // URLs, so force the browser to revalidate on every request.
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            axum::http::header::HeaderValue::from_static("no-cache"),
        ))
        // Compress static assets (notably the multi-megabyte WASM bundle)
        // on the fly when the client advertises `Accept-Encoding: gzip`.
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(conf.leptos_options);

    let (host, port_str) = addr.rsplit_once(':').unwrap_or((&addr, "3000"));
    let mut port: u16 = port_str.parse().unwrap_or(3000);
    for _ in 0..100 {
        let bind_addr = format!("{host}:{port}");
        if let Ok(listener) = tokio::net::TcpListener::bind(&bind_addr).await {
            tracing::info!("listening on http://{host}:{port}");
            let local = LocalSet::new();
            let _guard = local.enter();
            axum::serve(listener, app.clone().into_make_service()).await?;
            return Ok(());
        }
        port += 1;
    }
    Err("Could not bind to any port in range".into())
}

/// Next.js-style dev overlay script (framework-injected). Shows a floating
/// MontRS button in the bottom-right of any MontRS app during
/// `montrs serve`/`watch`, surfaces runtime errors without opening the
/// console, and reports live-reload connection status.
#[cfg(feature = "ssr")]
const DEV_OVERLAY_SCRIPT: &str = concat!(
    "<script>",
    r###"(function(){
if (window.__montrsDevOverlay) return; window.__montrsDevOverlay = 1;
var E = [];
function push(k, m, f) { E.push([k, m, f || '']); if (E.length > 60) E.shift(); }
window.addEventListener('error', function (e) { push('error', (e && e.message) || String(e.error || 'Error')); });
window.addEventListener('unhandledrejection', function (e) { var r = e && e.reason; push('rejection', r ? String(r) : 'Promise rejected'); });
try { (function (ce) { console.error = function () { push('console', Array.prototype.map.call(arguments, String).join(' ')); return ce.apply(console, arguments); }; })(console.error); } catch (_) {}
function dark() { return document.documentElement.classList.contains('dark'); }
function css() {
  var d = dark();
  var bg = d ? '#1a1a1a' : '#ffffff';
  var fg = d ? '#e5e5e5' : '#111111';
  var bd = d ? '#3a3a3a' : 'rgba(0,0,0,0.12)';
  return {
    btn: 'position:fixed;bottom:16px;right:16px;z-index:2147483000;width:40px;height:40px;border-radius:9999px;border:1px solid ' + bd + ';background:' + bg + ';box-shadow:0 8px 24px rgba(0,0,0,0.25);display:flex;align-items:center;justify-content:center;cursor:pointer;',
    panel: 'position:fixed;bottom:64px;right:16px;z-index:2147483000;width:380px;max-height:70vh;overflow:auto;background:' + bg + ';border:1px solid ' + bd + ';border-radius:10px;box-shadow:0 12px 40px rgba(0,0,0,0.3);font:11px/1.5 ui-monospace,Menlo,monospace;color:' + fg + ';',
    frame: 'display:block;margin:8px 0 12px;padding:10px;background:#0c0c0c;border:1px solid #2a2a2a;border-radius:6px;white-space:pre-wrap;word-break:break-word;color:#fca5a5;max-height:220px;overflow:auto;',
    copy: 'position:absolute;top:6px;right:6px;padding:2px 8px;font-size:10px;background:#2a2a2a;color:#e5e5e5;border:1px solid #3a3a3a;border-radius:4px;cursor:pointer;'
  };
}
var live = '○', busy = false;
var btn = document.createElement('button');
btn.title = 'MontRS dev console';
btn.style.cssText = css().btn;
var logo = null;
try { var l = document.querySelector('link[rel="icon"]'); if (l && l.href) logo = l.href; } catch (_) {}
if (logo) {
  btn.style.backgroundImage = 'url(' + logo + ')';
  btn.style.backgroundSize = '64%';
  btn.style.backgroundRepeat = 'no-repeat';
  btn.style.backgroundPosition = 'center';
} else {
  btn.innerHTML = '<svg width="22" height="22" viewBox="0 0 24 24"><rect x="1" y="1" width="22" height="22" rx="6" fill="none" stroke="#ff6310" stroke-width="2"/><path d="M7 17 V7 L12 13 L17 7 V17" fill="none" stroke="#ff6310" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>';
}
var panel = null, open = false;
function badge(k){ return k==='error' ? '#e5484d' : k==='rejection' ? '#b7791f' : '#60a5fa'; }
function esc(s){ return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'); }
function render() {
  if (!panel) return;
  var sep = dark() ? '#333' : '#eee';
  var out = '<div style="display:flex;justify-content:space-between;align-items:center;padding:8px 10px;border-bottom:1px solid ' + sep + '"><b>MontRS dev</b><span style="opacity:0.8">' + (busy ? 'compiling…' : (live === '●' ? 'live' : 'disconnected')) + '</span></div>';
  if (E.length === 0) {
    out += '<div style="padding:10px;opacity:0.7">No errors. Edits reload after the build.</div>';
  } else {
    for (var i = 0; i < E.length; i++) {
      var c = badge(E[i][0]);
      out += '<div style="padding:6px 10px;border-bottom:1px solid ' + (dark() ? '#2a2a2a' : '#f0f0f0') + ';color:' + c + ';white-space:pre-wrap;word-break:break-word">' + esc(E[i][1]) + '</div>';
      if (E[i][2]) {
        out += '<pre style="position:relative">' + esc(E[i][2]) + '<button class="copy" style="' + css().copy + '" data-i="' + i + '">copy</button></pre>';
      }
    }
  }
  panel.innerHTML = out;
  panel.querySelectorAll && panel.querySelectorAll('.copy').forEach(function (b) {
    b.onclick = function () {
      var i = parseInt(b.getAttribute('data-i'), 10);
      var t = E[i] && E[i][2] || '';
      try { navigator.clipboard.writeText(t); b.textContent = 'copied'; } catch (_) { b.textContent = 'fail'; }
    };
  });
}
btn.onclick = function () {
  open = !open;
  if (open) {
    if (!panel) { panel = document.createElement('div'); document.body.appendChild(panel); }
    panel.style.cssText = css().panel;
    render();
  } else if (panel) { panel.remove(); panel = null; }
};
document.body.appendChild(btn);
var port = '3001';
try { var m = document.querySelector('meta[name="montrs:reload-port"]'); if (m && m.content) port = m.content; } catch (_) {}
try {
  var ws = new WebSocket('ws://' + (location.hostname || 'localhost') + ':' + port);
  ws.onopen = function () { live = '●'; if (open) render(); };
  ws.onclose = function () { live = '○'; if (open) render(); };
  ws.onmessage = function (e) {
    var data; try { data = JSON.parse(e.data); } catch (_) { return; }
    if (data.type === 'building') { busy = true; if (open) render(); }
    else if (data.type === 'build-ok') { busy = false; live = '●'; if (open) render(); }
    else if (data.type === 'build-error') { busy = false; push('build', data.message || 'Build error', data.frame || ''); if (open) render(); }
    else if (data.type === 'server-error') { busy = false; push('server', data.message || 'Server error'); if (open) render(); }
  };
} catch (_) {}
})();
"###,
    "</script>"
);

/// Axum middleware: injects the dev overlay script + a meta tag with the
/// actual reload port into HTML responses so every MontRS app gets it in
/// dev, without touching the app's own code.
#[cfg(feature = "ssr")]
async fn inject_dev_overlay(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use futures::StreamExt;

    let res = next.run(req).await;
    let is_html = res
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.starts_with("text/html"));
    if !is_html {
        return res;
    }

    let port = std::env::var("MONTRS_RELOAD_PORT")
        .unwrap_or_else(|_| "3001".to_string());
    let meta_tag = format!(
        "<meta name=\"montrs:reload-port\" content=\"{}\">",
        port
    );
    let script = DEV_OVERLAY_SCRIPT;
    let injection = format!("{meta_tag}\n{script}");

    let headers = res.headers().clone();
    let body = res.into_body();
    let stream = body.into_data_stream();
    let script_bytes = futures::stream::once(async move {
        Ok::<_, axum::Error>(axum::body::Bytes::from(injection))
    });
    let merged = stream.chain(script_bytes);
    let mut new_res =
        axum::response::Response::new(axum::body::Body::from_stream(merged));
    *new_res.headers_mut() = headers;
    new_res
        .headers_mut()
        .remove(axum::http::header::CONTENT_LENGTH);
    new_res
}
