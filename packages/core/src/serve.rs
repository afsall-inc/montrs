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
    use tokio::task::LocalSet;

    let addr = std::env::var("MONTRS_SITE_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let (options, site_root) = configure_leptos_options();
    let app = build_axum_app(router, app_fn, options, &site_root);

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

/// Derive Leptos runtime configuration from `MONTRS_*` env vars.
#[cfg(feature = "ssr")]
fn configure_leptos_options() -> (leptos::prelude::LeptosOptions, String) {
    use leptos::prelude::*;

    let site_root = std::env::var("MONTRS_SITE_ROOT")
        .unwrap_or_else(|_| "target/site".to_string());
    let pkg_dir = std::env::var("MONTRS_SITE_PKG_DIR")
        .unwrap_or_else(|_| "pkg".to_string());
    let output_name = std::env::var("MONTRS_OUTPUT_NAME")
        .unwrap_or_else(|_| "website".to_string());
    let addr = std::env::var("MONTRS_SITE_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
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
    (conf.leptos_options, site_root)
}

/// Build the axum app that serves the app's routes, static files, the dev
/// overlay, and the `/_dioxus` bridge.
#[cfg(feature = "ssr")]
fn build_axum_app<C, F, IV>(
    router: Router<C>,
    app_fn: F,
    options: leptos::prelude::LeptosOptions,
    site_root: &str,
) -> axum::Router
where
    C: AppConfig + 'static,
    F: Fn() -> IV + Clone + Send + Sync + 'static,
    IV: leptos::prelude::IntoView + 'static,
{
    use axum::Router as AxumRouter;
    use leptos::prelude::*;
    use leptos_axum::LeptosRoutes;
    use tower_http::services::ServeDir;

    let axum_routes = router.to_axum_route_listings();
    let mut app = AxumRouter::new()
        .leptos_routes_with_context(
            &options,
            axum_routes,
            {
                let r = router.clone();
                let leptos_options = options.clone();
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
        .fallback_service(ServeDir::new(site_root));

    // Next.js-style dev overlay, injected by the framework itself (not the
    // app) so it appears in every MontRS app during `montrs serve`/`watch`.
    // `/_dioxus` bridges to the CLI's hot-patch socket so a Leptos
    // `connect_to_hot_patch_messages` client (which targets that path) works.
    if std::env::var("LEPTOS_WATCH").is_ok() {
        app = app
            .route("/_dioxus", axum::routing::get(devtools_bridge))
            .layer(axum::middleware::from_fn(inject_dev_overlay));
    }

    app
        // SSR renders once and needs no reactivity, so signal reads during
        // rendering are the intended false positives the non-reactive zone
        // covers; this silences Leptos's debug-mode untracked-read warnings.
        .layer(axum::middleware::from_fn(non_reactive_zone))
        // Dev servers must never serve stale bundles. The hydration entry
        // (`/pkg/front.js`, `/pkg/front_bg.wasm`) and stylesheets use fixed
        // URLs, so Chrome's WASM/JS code caches keep serving old bytes unless
        // we forbid storing entirely. `no-store` (unlike `no-cache`) also
        // defeats the back/forward and disk caches.
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            axum::http::header::HeaderValue::from_static(
                "no-store, no-cache, must-revalidate",
            ),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::PRAGMA,
            axum::http::header::HeaderValue::from_static("no-cache"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::EXPIRES,
            axum::http::header::HeaderValue::from_static("0"),
        ))
        // Compress static assets (notably the multi-megabyte WASM bundle)
        // on the fly when the client advertises `Accept-Encoding: gzip`.
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(options)
}

/// A request to render through [`SsrApp`] without binding a socket.
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Default)]
pub struct SsrRequest {
    /// HTTP method (defaults to `GET`).
    pub method: String,
    /// Request URI, e.g. `/api/users?active=true`.
    pub uri: String,
    /// Request headers.
    pub headers: Vec<(String, String)>,
    /// Request body bytes.
    pub body: Vec<u8>,
}

#[cfg(feature = "ssr")]
impl SsrRequest {
    /// A `GET` request for `uri`.
    pub fn get(uri: impl Into<String>) -> Self {
        Self {
            method: "GET".to_string(),
            uri: uri.into(),
            ..Default::default()
        }
    }

    /// A request with an explicit method and `uri`.
    pub fn new(method: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            uri: uri.into(),
            ..Default::default()
        }
    }

    /// Set the request body.
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// Set a JSON body and `content-type` header.
    pub fn with_json<T: serde::Serialize>(
        mut self,
        value: &T,
    ) -> Result<Self, serde_json::Error> {
        self.body = serde_json::to_vec(value)?;
        self.headers
            .push(("content-type".to_string(), "application/json".to_string()));
        Ok(self)
    }

    /// Add a header.
    pub fn with_header(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

/// An SSR app that can render individual requests **without** binding a socket.
///
/// Used by the dev shell's hot-swappable app dylib: the shell owns the HTTP
/// listener and calls [`SsrApp::render`] per request, so swapping the dylib
/// never drops the listener.
#[cfg(feature = "ssr")]
pub struct SsrApp {
    /// The constructed axum app.
    pub app: axum::Router,
    /// The Leptos options the app was built with.
    pub options: leptos::prelude::LeptosOptions,
}

#[cfg(feature = "ssr")]
impl SsrApp {
    /// Build an SSR app from an app router and root view.
    pub fn build<C, F, IV>(
        router: Router<C>,
        app_fn: F,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        C: AppConfig + 'static,
        F: Fn() -> IV + Clone + Send + Sync + 'static,
        IV: leptos::prelude::IntoView + 'static,
    {
        let (options, site_root) = configure_leptos_options();
        let app = build_axum_app(router, app_fn, options.clone(), &site_root);
        Ok(Self { app, options })
    }

    /// Render a single request to `(status, headers, body)`.
    pub fn render(
        &self,
        method: &str,
        uri: &str,
    ) -> Result<(u16, Vec<(String, String)>, Vec<u8>), Box<dyn std::error::Error>>
    {
        self.render_request(SsrRequest::new(method, uri))
    }

    /// Render a full request (method, URI, headers, body) without a socket.
    pub fn render_request(
        &self,
        request: SsrRequest,
    ) -> Result<(u16, Vec<(String, String)>, Vec<u8>), Box<dyn std::error::Error>>
    {
        use axum::body::Body;
        use tower::ServiceExt;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let local = tokio::task::LocalSet::new();
        let app = self.app.clone();
        let method = axum::http::Method::from_bytes(request.method.as_bytes())?;
        let uri = request.uri.clone();

        rt.block_on(local.run_until(async move {
            let mut builder =
                axum::http::Request::builder().method(method).uri(uri);
            for (name, value) in &request.headers {
                builder = builder.header(name.as_str(), value.as_str());
            }
            let req = builder.body(Body::from(request.body))?;
            let res = app.oneshot(req).await?;
            let status = res.status().as_u16();
            let headers = res
                .headers()
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_str().to_string(),
                        v.to_str().unwrap_or_default().to_string(),
                    )
                })
                .collect();
            let body = axum::body::to_bytes(res.into_body(), usize::MAX)
                .await?
                .to_vec();
            Ok::<_, Box<dyn std::error::Error>>((status, headers, body))
        }))
    }
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
// One-time dev cleanup: stale service workers / Cache Storage from earlier
// iterations pin old bundles in Chrome even across a hard refresh.
try { if (navigator.serviceWorker && navigator.serviceWorker.getRegistrations) { navigator.serviceWorker.getRegistrations().then(function (rs) { rs.forEach(function (r) { r.unregister(); }); }); } if (window.caches && caches.keys) { caches.keys().then(function (ks) { ks.forEach(function (k) { caches.delete(k); }); }); } } catch (_) {}
var E = [];
function hasError() {
  for (var i = 0; i < E.length; i++) {
    var k = E[i][0];
    if (k === 'error' || k === 'rejection' || k === 'build' || k === 'server') return true;
  }
  return false;
}
function hasWarn() {
  for (var i = 0; i < E.length; i++) {
    var k = E[i][0];
    if (k === 'warn' || k === 'console' || k === 'warning') return true;
  }
  return false;
}
function push(k, m, f) {
  E.push([k, m, f || '']);
  if (E.length > 60) E.shift();
  if (typeof updateRing === 'function') updateRing();
  if (open && typeof render === 'function') render();
}
window.addEventListener('error', function (e) { push('error', (e && e.message) || String(e.error || 'Error')); });
window.addEventListener('unhandledrejection', function (e) { var r = e && e.reason; push('rejection', r ? String(r) : 'Promise rejected'); });
try { (function (ce, cw) {
  console.error = function () { push('console', Array.prototype.map.call(arguments, String).join(' ')); return ce.apply(console, arguments); };
  if (cw) { console.warn = function () { push('warn', Array.prototype.map.call(arguments, String).join(' ')); return cw.apply(console, arguments); }; }
})(console.error, console.warn); } catch (_) {}
// User preferences: which side the button floats on, and how opaque it is.
// Never fully transparent, so it can always be found and reopened.
var MIN_OPACITY = 0.2;
var settings = { pos: 'right', opacity: 1 };
try {
  var stored = localStorage.getItem('montrs-dev-overlay');
  if (stored) {
    var sp = JSON.parse(stored);
    if (sp && (sp.pos === 'left' || sp.pos === 'right')) settings.pos = sp.pos;
    if (sp && typeof sp.opacity === 'number' && isFinite(sp.opacity)) {
      settings.opacity = Math.min(1, Math.max(MIN_OPACITY, sp.opacity));
    }
  }
} catch (_) {}
function saveSettings() { try { localStorage.setItem('montrs-dev-overlay', JSON.stringify(settings)); } catch (_) {} }
function dark() { return document.documentElement.classList.contains('dark'); }
function sideCss() { return settings.pos === 'left' ? 'left:16px;' : 'right:16px;'; }
function css() {
  var d = dark();
  var bg = d ? '#1a1a1a' : '#ffffff';
  var fg = d ? '#e5e5e5' : '#111111';
  var bd = d ? '#3a3a3a' : 'rgba(0,0,0,0.12)';
  var op = settings.opacity;
  return {
    btn: 'position:fixed;bottom:16px;' + sideCss() + 'z-index:2147483000;width:40px;height:40px;border-radius:9999px;border:1px solid ' + bd + ';background:' + bg + ';box-shadow:0 8px 24px rgba(0,0,0,0.25);display:flex;align-items:center;justify-content:center;cursor:pointer;opacity:' + op + ';transition:opacity .15s ease;',
    panel: 'position:fixed;bottom:64px;' + sideCss() + 'z-index:2147483000;width:380px;max-width:calc(100vw - 32px);max-height:70vh;overflow:auto;background:' + bg + ';border:1px solid ' + bd + ';border-radius:10px;box-shadow:0 12px 40px rgba(0,0,0,0.3);font:11px/1.5 ui-monospace,Menlo,monospace;color:' + fg + ';opacity:' + op + ';',
    frame: 'display:block;margin:8px 0 12px;padding:10px;background:#0c0c0c;border:1px solid #2a2a2a;border-radius:6px;white-space:pre-wrap;word-break:break-word;color:#fca5a5;max-height:220px;overflow:auto;',
    copy: 'position:absolute;top:6px;right:6px;padding:2px 8px;font-size:10px;background:#2a2a2a;color:#e5e5e5;border:1px solid #3a3a3a;border-radius:4px;cursor:pointer;',
    ctrl: 'padding:3px 8px;font-size:10px;background:transparent;color:' + fg + ';border:1px solid ' + bd + ';border-radius:4px;cursor:pointer;'
  };
}
var live = 'connecting', busy = false;
var btn = document.createElement('button');
btn.title = 'MontRS dev console — open to move it or change opacity';
var logo = null;
try { var l = document.querySelector('link[rel="icon"]'); if (l && l.href) logo = l.href; } catch (_) {}
if (!logo) {
  btn.innerHTML = '<svg width="22" height="22" viewBox="0 0 24 24"><rect x="1" y="1" width="22" height="22" rx="6" fill="none" stroke="#ff6310" stroke-width="2"/><path d="M7 17 V7 L12 13 L17 7 V17" fill="none" stroke="#ff6310" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>';
}
// Red ring when anything failed, amber ring when only warnings are present,
// no ring when the log is clean. Drawn as an outline so it survives the logo
// background and stays visible at any opacity.
function updateRing() {
  if (!btn) return;
  var err = hasError(), warn = hasWarn();
  var ring = err ? '#e5484d' : (warn ? '#d29922' : '');
  btn.style.outline = ring ? '2px solid ' + ring : 'none';
  btn.style.outlineOffset = '2px';
  var shadow = '0 8px 24px rgba(0,0,0,0.25)';
  if (ring) shadow += ',0 0 0 4px ' + (err ? 'rgba(229,72,77,0.25)' : 'rgba(210,153,34,0.25)');
  btn.style.boxShadow = shadow;
  var n = err ? 'error(s)' : (warn ? 'warning(s)' : '');
  btn.title = 'MontRS dev console' + (n ? ' — ' + n + ' logged' : '') + ' — open to move it or change opacity';
}
function styleBtn() {
  btn.style.cssText = css().btn;
  if (logo) {
    btn.style.backgroundImage = 'url(' + logo + ')';
    btn.style.backgroundSize = '64%';
    btn.style.backgroundRepeat = 'no-repeat';
    btn.style.backgroundPosition = 'center';
  }
  updateRing();
}
styleBtn();
var panel = null, open = false;
function badge(k){
  if (k === 'error' || k === 'build' || k === 'server') return '#e5484d';
  if (k === 'rejection' || k === 'warn' || k === 'warning') return '#d29922';
  return '#60a5fa';
}
function esc(s){ return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'); }
function copyAllText() {
  return E.map(function (e) {
    var tag = '[' + e[0].toUpperCase() + '] ';
    var body = e[1] || '';
    if (e[2]) body += '\n' + e[2];
    return tag + body;
  }).join('\n\n');
}
function render() {
  if (!panel) return;
  var sep = dark() ? '#333' : '#eee';
  var status = busy ? 'compiling…' : (live === 'live' ? 'live' : (live === 'connecting' ? 'connecting…' : 'disconnected'));
  // Surface the reload port while not connected so a mismatch is obvious.
  if (live !== 'live') status += ' (:' + port + ')';
  var dot = live === 'live' ? '#3fb950' : (live === 'connecting' ? '#d29922' : '#e5484d');
  var out = '<div style="display:flex;justify-content:space-between;align-items:center;padding:8px 10px;border-bottom:1px solid ' + sep + '"><b>MontRS dev</b><span style="opacity:0.9;display:inline-flex;align-items:center;gap:6px"><span style="color:' + dot + '">●</span>' + status + '</span></div>';
  // Appearance controls: which side it floats on, and how see-through it is.
  out += '<div style="display:flex;align-items:center;gap:8px;padding:6px 10px;border-bottom:1px solid ' + sep + '">'
    + '<button class="pos-toggle" title="Move the console to the other side" style="' + css().ctrl + '">'
    + (settings.pos === 'left' ? 'Right ▸' : '◂ Left') + '</button>'
    + '<span style="opacity:0.7;white-space:nowrap">Opacity</span>'
    + '<input class="op" type="range" min="' + MIN_OPACITY + '" max="1" step="0.05" value="' + settings.opacity + '" style="flex:1;accent-color:#ff6310" title="Drag to make the console more transparent (min ' + Math.round(MIN_OPACITY * 100) + '%)">';
  if (E.length > 0) {
    out += '<button class="copy-all" title="Copy all logged messages and frames" style="' + css().ctrl + ';margin-left:auto">Copy all</button>';
  }
  out += '</div>';
  if (E.length === 0) {
    out += '<div style="padding:10px;opacity:0.7">No errors. Edits reload after the build.</div>';
  } else {
    for (var i = 0; i < E.length; i++) {
      var c = badge(E[i][0]);
      var kind = E[i][0];
      out += '<div style="padding:6px 10px;border-bottom:1px solid ' + (dark() ? '#2a2a2a' : '#f0f0f0') + '">';
      out += '<div style="display:flex;align-items:center;justify-content:space-between;gap:6px;margin-bottom:2px">';
      out += '<span style="font-weight:600;font-size:10px;text-transform:uppercase;color:' + c + '">' + esc(kind) + '</span>';
      out += '<button class="copy-item" data-i="' + i + '" style="' + css().ctrl + ';padding:1px 6px;font-size:9px" title="Copy this message">copy</button>';
      out += '</div>';
      out += '<div style="color:' + c + ';white-space:pre-wrap;word-break:break-word">' + esc(E[i][1]) + '</div>';
      if (E[i][2]) {
        out += '<pre style="position:relative;margin:6px 0 0;' + css().frame + '">' + esc(E[i][2]) + '<button class="copy-frame" style="' + css().copy + '" data-i="' + i + '">copy frame</button></pre>';
      }
      out += '</div>';
    }
  }
  panel.innerHTML = out;
  panel.querySelectorAll && panel.querySelectorAll('.copy-item').forEach(function (b) {
    b.onclick = function () {
      var i = parseInt(b.getAttribute('data-i'), 10);
      var item = E[i];
      if (!item) return;
      var text = '[' + item[0].toUpperCase() + '] ' + item[1] + (item[2] ? '\n\n' + item[2] : '');
      try { navigator.clipboard.writeText(text); b.textContent = 'copied'; setTimeout(function(){ b.textContent = 'copy'; }, 1500); }
      catch (_) { b.textContent = 'fail'; }
    };
  });
  panel.querySelectorAll && panel.querySelectorAll('.copy-frame').forEach(function (b) {
    b.onclick = function () {
      var i = parseInt(b.getAttribute('data-i'), 10);
      var t = E[i] && E[i][2] || '';
      try { navigator.clipboard.writeText(t); b.textContent = 'copied'; setTimeout(function(){ b.textContent = 'copy frame'; }, 1500); }
      catch (_) { b.textContent = 'fail'; }
    };
  });
  var copyAllBtn = panel.querySelector('.copy-all');
  if (copyAllBtn) {
    copyAllBtn.onclick = function () {
      var all = copyAllText();
      try { navigator.clipboard.writeText(all); copyAllBtn.textContent = 'Copied!'; setTimeout(function(){ copyAllBtn.textContent = 'Copy all'; }, 1500); }
      catch (_) { copyAllBtn.textContent = 'Failed'; }
    };
  }
  var op = panel.querySelector('.op');
  if (op) {
    op.oninput = function () {
      settings.opacity = Math.min(1, Math.max(MIN_OPACITY, parseFloat(op.value) || 1));
      saveSettings();
      btn.style.opacity = settings.opacity;
      panel.style.opacity = settings.opacity;
    };
  }
  var posBtn = panel.querySelector('.pos-toggle');
  if (posBtn) {
    posBtn.onclick = function () {
      settings.pos = settings.pos === 'left' ? 'right' : 'left';
      saveSettings();
      styleBtn();
      panel.style.cssText = css().panel;
      render();
    };
  }
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
// Reload once the restarted SSR server answers again (it is briefly offline
// while the child is swapped after a successful build).
function reloadWhenReady(n) {
  // Only reload once the real server answers. The "compiling…" fallback
  // responds 503, so an `ok` check prevents getting stuck on it.
  fetch(location.href, { cache: 'no-store' }).then(function (r) {
    if (r.ok) location.reload();
    else if (n > 0) setTimeout(function () { reloadWhenReady(n - 1); }, 400);
  }).catch(function () { if (n > 0) setTimeout(function () { reloadWhenReady(n - 1); }, 400); else location.reload(); });
}
// Try the page host first, then loopback in case the hostname does not
// resolve to the interface the reload server is bound to.
var hosts = [location.hostname || 'localhost'];
if (hosts.indexOf('127.0.0.1') === -1) hosts.push('127.0.0.1');
var hostIdx = 0;
function connect() {
  var host = hosts[hostIdx % hosts.length];
  var ws;
  try { ws = new WebSocket('ws://' + host + ':' + port); }
  catch (_) { hostIdx++; setTimeout(connect, 1000); return; }
  ws.onopen = function () {
    hostIdx = 0;
    live = 'live'; if (open) render();
    // Identifies this socket as the overlay so the server routes dev events
    // (building/build-ok/build-error) here instead of reload frames.
    try { ws.send('{"hello":"montrs-overlay"}'); } catch (_) {}
  };
  ws.onclose = function () { live = 'off'; if (open) render(); hostIdx++; setTimeout(connect, 1000); };
  ws.onerror = function () { try { ws.close(); } catch (_) {} };
  ws.onmessage = function (e) {
    var data; try { data = JSON.parse(e.data); } catch (_) { return; }
    // Leptos view! patch: apply markup changes to the live DOM in place.
    if (data.view) { try { patch(data.view); } catch (_) {} return; }
    // CSS swap: bust the stylesheet URL without a reload.
    if (data.css) {
      try {
        document.querySelectorAll('link[rel="stylesheet"]').forEach(function (l) {
          var href = l.getAttribute('href') || '';
          if (href.indexOf(data.css) !== -1) {
            l.setAttribute('href', '/' + data.css + '?v=' + Date.now());
          }
        });
      } catch (_) {}
      return;
    }
    if (data.type === 'building') { busy = true; if (open) render(); }
    else if (data.type === 'build-ok') {
      busy = false; live = 'live';
      if (open) render();
      reloadWhenReady(25);
    }
    else if (data.type === 'build-error') { busy = false; push('build', data.message || 'Build error', data.frame || ''); if (open) render(); }
    else if (data.type === 'server-error') { busy = false; push('server', data.message || 'Server error'); if (open) render(); }
  };
}
connect();
})();
"###,
    "</script>"
);

/// Suppress Leptos's debug-mode "outside a reactive tracking context" warnings
/// while SSR renders. The server renders once and needs no reactivity, so these
/// reads are the false positives the non-reactive zone is meant to cover.
#[cfg(feature = "ssr")]
async fn non_reactive_zone(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    reactive_graph::diagnostics::SpecialNonReactiveFuture::new(next.run(req))
        .await
}

/// Upgrade `/_dioxus` and bridge it to the CLI's hot-patch socket
/// (`MONTRS_HOTPATCH_ADDR`), so a client targeting that path reaches the hub.
#[cfg(feature = "ssr")]
async fn devtools_bridge(
    upgrade: axum::extract::WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    upgrade.on_upgrade(devtools_socket)
}

#[cfg(feature = "ssr")]
async fn devtools_socket(client: axum::extract::ws::WebSocket) {
    use futures::{SinkExt, StreamExt};

    let Ok(addr) = std::env::var("MONTRS_HOTPATCH_ADDR") else {
        return;
    };
    let Ok((upstream, _)) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/")).await
    else {
        return;
    };

    let (mut client_tx, mut client_rx) = client.split();
    let (mut up_tx, mut up_rx) = upstream.split();

    let to_upstream = tokio::spawn(async move {
        while let Some(Ok(msg)) = client_rx.next().await {
            let out = match msg {
                axum::extract::ws::Message::Text(text) => {
                    tokio_tungstenite::tungstenite::Message::Text(
                        text.to_string().into(),
                    )
                }
                axum::extract::ws::Message::Close(_) => break,
                _ => continue,
            };
            if up_tx.send(out).await.is_err() {
                break;
            }
        }
    });

    let to_client = tokio::spawn(async move {
        while let Some(Ok(msg)) = up_rx.next().await {
            let out = match msg {
                tokio_tungstenite::tungstenite::Message::Text(text) => {
                    axum::extract::ws::Message::Text(text.to_string().into())
                }
                tokio_tungstenite::tungstenite::Message::Close(_) => break,
                _ => continue,
            };
            if client_tx.send(out).await.is_err() {
                break;
            }
        }
    });

    let _ = tokio::join!(to_upstream, to_client);
}

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
    let meta_tag =
        format!("<meta name=\"montrs:reload-port\" content=\"{}\">", port);
    // `HOT_RELOAD_JS` defines the global `patch(json)` that applies Leptos
    // `view!` patches to the live DOM. Injected here so every MontRS app gets
    // view hot-reload with no app changes.
    let hot_reload =
        format!("<script>{}</script>", leptos_hot_reload::HOT_RELOAD_JS);
    let injection = format!("{meta_tag}\n{hot_reload}\n{DEV_OVERLAY_SCRIPT}");

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
