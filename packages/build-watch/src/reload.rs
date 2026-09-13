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

//! Live-reload + dev-tools WebSocket for `montrs serve` / `montrs watch`.
//!
//! Listens on the project's `[serve] reload-port` (default 3001). Two kinds
//! of clients connect:
//!
//! * the Leptos reload script, which expects JSON reload frames
//!   (`{"all":true}` / `{"css":"…"}`) and reloads on any message it receives;
//! * the MontRS dev overlay, which sends `{"hello":"montrs-overlay"}` first and
//!   then only wants structured build/server events.
//!
//! Connections are tagged from their hello frame so Leptos never receives a
//! dev-tools event (which it would treat as a reload) and the overlay never
//! receives a reload frame.

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{net::TcpListener, sync::broadcast};

/// A tiny WS server that fans out dev events to every connected browser tab.
///
/// It also remembers the last *pending* dev event (`building` / `build-error`
/// / `server-error`) so a client that connects mid-build or after a failure is
/// immediately told the real state instead of showing a stale "live".
#[derive(Clone)]
pub struct LiveReload {
    tx: broadcast::Sender<String>,
    last_event: Arc<Mutex<Option<String>>>,
}

impl LiveReload {
    /// Bind a live-reload port and start accepting connections, returning the
    /// bound port alongside the server.
    ///
    /// The configured (`preferred`) port is used when free; otherwise the next
    /// two ports are tried, then an ephemeral one. A leftover dev server still
    /// holding the port must not leave the browser overlay silently
    /// "disconnected" — the caller forwards the returned port to the SSR child
    /// (and into the injected meta tag) so the browser connects to the right
    /// place.
    pub async fn start(preferred: u16) -> Result<(Self, u16)> {
        let (tx, _rx) = broadcast::channel::<String>(64);
        let last_event: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        let mut listener = None;
        let mut actual_port = preferred;
        for candidate in [
            preferred,
            preferred.saturating_add(1),
            preferred.saturating_add(2),
            0,
        ] {
            let addr = SocketAddr::from(([0, 0, 0, 0], candidate));
            match TcpListener::bind(addr).await {
                Ok(l) => {
                    actual_port =
                        l.local_addr().map(|a| a.port()).unwrap_or(candidate);
                    listener = Some(l);
                    break;
                }
                Err(_) => continue,
            }
        }
        let listener = listener.ok_or_else(|| {
            anyhow::anyhow!("could not bind any live-reload port")
        })?;

        let tx2 = tx.clone();
        let last2 = last_event.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _peer)) = listener.accept().await else {
                    continue;
                };
                let tx = tx2.clone();
                let last = last2.clone();
                tokio::spawn(async move {
                    let Ok(mut ws) = tokio_tungstenite::accept_async(stream).await
                    else {
                        return;
                    };

                    // Read the hello frame (if any) to tag this connection.
                    let hello = tokio::time::timeout(
                        Duration::from_millis(750),
                        ws.next(),
                    )
                    .await
                    .ok()
                    .flatten()
                    .and_then(|m| m.ok())
                    .and_then(|m| m.into_text().ok())
                    .unwrap_or_default();
                    let is_overlay = hello.contains("montrs-overlay");

                    // Overlay clients joining mid-build (or after a failure)
                    // get the current pending state right away. `build-ok` is
                    // deliberately never replayed — it would reload-loop.
                    if is_overlay {
                        use tokio_tungstenite::tungstenite::Message;
                        let pending = last.lock().ok().and_then(|g| g.clone());
                        if let Some(msg) = pending
                            && ws.send(Message::Text(msg.into())).await.is_err()
                        {
                            return;
                        }
                    }

                    let mut rx = tx.subscribe();
                    while let Ok(msg) = rx.recv().await {
                        use tokio_tungstenite::tungstenite::Message;
                        // Dev-tools frames always carry a `"type"` field and go
                        // only to the overlay. Reload frames (`all` / `view` /
                        // `css`) go to every client, so the overlay can apply
                        // view patches and CSS swaps itself.
                        let is_event = msg.contains("\"type\"");
                        if (!is_event || is_overlay)
                            && ws.send(Message::Text(msg.into())).await.is_err()
                        {
                            break;
                        }
                    }
                });
            }
        });

        Ok((Self { tx, last_event }, actual_port))
    }

    fn send_text(&self, text: String) {
        let _ = self.tx.send(text);
    }

    /// Remember the last pending state; `None` clears it (build finished).
    fn set_pending(&self, text: Option<String>) {
        if let Ok(mut last) = self.last_event.lock() {
            *last = text;
        }
    }

    /// Ask every connected browser tab to reload — the Leptos reload script
    /// expects JSON (`{"all": true}` → full reload).
    pub fn notify(&self) {
        self.send_text(r#"{"all":true}"#.to_string());
    }

    /// Hot-swap a stylesheet without a full reload (`{"css":"main.css"}`).
    pub fn notify_css(&self, css: &str) {
        self.send_text(format!(r#"{{"css":"{css}"}}"#));
    }

    /// Send Leptos `view!` patches (`{"view":"<Patches json>"}`) so the browser
    /// can apply markup changes without a recompile or reload.
    pub fn view(&self, payload: String) {
        self.send_text(json!({ "view": payload }).to_string());
    }

    /// A rebuild started.
    pub fn building(&self) {
        let msg = json!({ "type": "building" }).to_string();
        self.set_pending(Some(msg.clone()));
        self.send_text(msg);
    }

    /// A rebuild succeeded with no errors.
    pub fn build_ok(&self) {
        // Nothing is pending once the build succeeds; clearing prevents a
        // replayed `build-ok` from reload-looping a fresh tab.
        self.set_pending(None);
        self.send_text(json!({ "type": "build-ok" }).to_string());
    }

    /// A build failed. `frame` is an optional pre-formatted code frame.
    pub fn build_error(
        &self,
        message: &str,
        file: Option<&str>,
        line: Option<u32>,
        column: Option<u32>,
        frame: Option<&str>,
    ) {
        let value = json!({
            "type": "build-error",
            "message": message,
            "file": file,
            "line": line,
            "column": column,
            "frame": frame,
        });
        let text = value.to_string();
        self.set_pending(Some(text.clone()));
        self.send_text(text);
    }

    /// The SSR server process failed to start or crashed.
    pub fn server_error(&self, message: &str) {
        let value = json!({ "type": "server-error", "message": message });
        let text = value.to_string();
        self.set_pending(Some(text.clone()));
        self.send_text(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_tungstenite::tungstenite::Message;

    type Ws = tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >;

    /// Read text frames until one contains `needle` (or fail loudly).
    async fn next_containing(ws: &mut Ws, needle: &str) -> String {
        for _ in 0..10 {
            match ws.next().await {
                Some(Ok(Message::Text(t))) => {
                    let s = t.to_string();
                    if s.contains(needle) {
                        return s;
                    }
                }
                Some(Ok(_)) => continue,
                other => panic!("unexpected ws message: {other:?}"),
            }
        }
        panic!("did not receive a message containing {needle}");
    }

    #[tokio::test]
    async fn overlay_receives_live_events_and_replayed_state() {
        let (reload, port) =
            LiveReload::start(0).await.expect("start reload server");
        let url = format!("ws://127.0.0.1:{port}/");

        // A build is already in progress before the overlay connects: it must
        // be told the pending state on connect.
        reload.building();

        let (mut ws, _) =
            tokio_tungstenite::connect_async(&url).await.expect("connect");
        ws.send(Message::Text("{\"hello\":\"montrs-overlay\"}".into()))
            .await
            .expect("send hello");

        let msg = next_containing(&mut ws, "building").await;
        assert!(msg.contains("\"type\":\"building\""), "{msg}");

        // ...and it must still receive live events afterwards.
        reload.build_ok();
        let msg = next_containing(&mut ws, "build-ok").await;
        assert!(msg.contains("build-ok"), "{msg}");
    }

    #[tokio::test]
    async fn non_overlay_client_only_receives_reload_frames() {
        let (reload, port) =
            LiveReload::start(0).await.expect("start reload server");
        let url = format!("ws://127.0.0.1:{port}/");

        // No hello => tagged as a Leptos reload client.
        let (mut ws, _) =
            tokio_tungstenite::connect_async(&url).await.expect("connect");
        // The server waits up to 750ms for a hello before subscribing.
        tokio::time::sleep(Duration::from_millis(900)).await;

        reload.building();
        reload.notify();

        let msg = next_containing(&mut ws, "\"all\":true").await;
        assert!(msg.contains("\"all\":true"), "{msg}");
        assert!(
            !msg.contains("\"type\""),
            "reload client received a dev event: {msg}"
        );
    }
}
