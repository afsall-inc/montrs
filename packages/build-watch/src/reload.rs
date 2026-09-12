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

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::{net::TcpListener, sync::broadcast};

/// A tiny WS server that fans out dev events to every connected browser tab.
#[derive(Clone)]
pub struct LiveReload {
    tx: broadcast::Sender<String>,
}

impl LiveReload {
    /// Bind `0.0.0.0:{port}` and start accepting connections.
    ///
    /// Fails gracefully if the port is already in use (e.g. another dev
    /// server is running) so the dev loop can keep going.
    pub async fn start(port: u16) -> Result<Self> {
        let (tx, _rx) = broadcast::channel::<String>(64);
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(addr).await.with_context(|| {
            format!("could not bind live-reload port {port}")
        })?;

        let tx2 = tx.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _peer)) = listener.accept().await else {
                    continue;
                };
                let tx = tx2.clone();
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

                    let mut rx = tx.subscribe();
                    while let Ok(msg) = rx.recv().await {
                        use tokio_tungstenite::tungstenite::Message;
                        // Dev-tools frames always carry a `"type"` field; reload
                        // frames never do. Route them to the right client.
                        let is_event = msg.contains("\"type\"");
                        if is_event == is_overlay
                            && ws.send(Message::Text(msg.into())).await.is_err()
                        {
                            break;
                        }
                    }
                });
            }
        });

        Ok(Self { tx })
    }

    fn send_text(&self, text: String) {
        let _ = self.tx.send(text);
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

    /// A rebuild started.
    pub fn building(&self) {
        self.send_text(json!({ "type": "building" }).to_string());
    }

    /// A rebuild succeeded with no errors.
    pub fn build_ok(&self) {
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
        self.send_text(value.to_string());
    }

    /// The SSR server process failed to start or crashed.
    pub fn server_error(&self, message: &str) {
        let value = json!({ "type": "server-error", "message": message });
        self.send_text(value.to_string());
    }
}
