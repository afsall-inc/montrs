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

//! Live-reload WebSocket broadcaster for `montrs serve` / `montrs watch`.
//!
//! Listens on the project's `[serve] reload-port` (default 3001). The SSR
//! shell's hydration scripts connect here and reload the page whenever the
//! watcher finishes a successful rebuild — the Leptos reload script reloads
//! on any incoming message, so we broadcast a simple "reload" text frame.

use anyhow::{Context, Result};
use futures_util::SinkExt;
use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::broadcast};

/// A tiny WS server that broadcasts reload notifications to every connected
/// browser tab.
#[derive(Clone)]
pub struct LiveReload {
    tx: broadcast::Sender<String>,
}

impl LiveReload {
    /// Bind `0.0.0.0:{port}` and start accepting reload connections.
    ///
    /// Fails gracefully if the port is already in use (e.g. another dev
    /// server is running) so the dev loop can keep going.
    pub async fn start(port: u16) -> Result<Self> {
        let (tx, _rx) = broadcast::channel::<String>(16);
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
                    let Ok(mut ws) =
                        tokio_tungstenite::accept_async(stream).await
                    else {
                        return;
                    };
                    let mut rx = tx.subscribe();
                    while let Ok(msg) = rx.recv().await {
                        use tokio_tungstenite::tungstenite::Message;
                        if ws.send(Message::Text(msg.into())).await.is_err() {
                            break;
                        }
                    }
                });
            }
        });

        Ok(Self { tx })
    }

    /// Ask every connected browser tab to reload — the Leptos `reload_script.js`
    /// handshake expects JSON (`{"all": true}` → full reload, `{"css": …}` →
    /// stylesheet swap), not a raw string.
    pub fn notify(&self) {
        let _ = self.tx.send(r#"{"all":true}"#.to_string());
    }

    /// Hot-swap a stylesheet without a full reload (`{"css":"main.css"}`).
    pub fn notify_css(&self, css: &str) {
        let _ = self.tx.send(format!(r#"{{"css":"{css}"}}"#));
    }
}
