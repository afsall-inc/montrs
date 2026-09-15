// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The hot-patch dev-server socket: clients connect, report their runtime base
//! address (ASLR reference), and receive [`DevserverMsg`]s — notably a
//! [`JumpTable`]-bearing [`crate::DevserverMsg::HotReload`] to apply.
//!
//! This speaks the same JSON `DevserverMsg`/`ClientMsg` shapes as the Dioxus
//! devtools protocol, so a Leptos app's `connect_to_hot_patch_messages` client
//! (or our overlay) can consume it.

use crate::{ClientMsg, DevserverMsg, HotReloadMsg, JumpTable};
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_tungstenite::tungstenite::Message;

/// The latest ASLR reference a client reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AslrReport {
    pub build_id: u64,
    pub pid: Option<u32>,
    pub aslr_reference: u64,
}

/// A broadcast hub for hot-patch messages.
#[derive(Clone)]
pub struct HotPatchServer {
    tx: broadcast::Sender<String>,
    aslr: Arc<Mutex<Option<AslrReport>>>,
}

impl HotPatchServer {
    /// Bind `0.0.0.0:{port}` (or the next two, then ephemeral) and accept
    /// clients. Returns the server and the port actually bound.
    pub async fn start(preferred: u16) -> Result<(Self, u16)> {
        let (tx, _rx) = broadcast::channel::<String>(64);
        let aslr: Arc<Mutex<Option<AslrReport>>> = Arc::new(Mutex::new(None));

        let mut listener = None;
        let mut actual_port = preferred;
        for candidate in [
            preferred,
            preferred.saturating_add(1),
            preferred.saturating_add(2),
            0,
        ] {
            match TcpListener::bind(("0.0.0.0", candidate)).await {
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
            anyhow::anyhow!("could not bind a hot-patch port")
        })?;

        let tx2 = tx.clone();
        let aslr2 = aslr.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _peer)) = listener.accept().await else {
                    continue;
                };
                let tx = tx2.clone();
                let aslr = aslr2.clone();
                tokio::spawn(async move {
                    let Ok(ws) = tokio_tungstenite::accept_async(stream).await
                    else {
                        return;
                    };
                    let (mut sink, mut source) = ws.split();
                    let mut rx = tx.subscribe();
                    loop {
                        tokio::select! {
                            outgoing = rx.recv() => {
                                let Ok(text) = outgoing else { continue };
                                if sink.send(Message::Text(text.into())).await.is_err() {
                                    break;
                                }
                            }
                            incoming = source.next() => {
                                match incoming {
                                    Some(Ok(Message::Text(text))) => {
                                        if let Ok(ClientMsg::AslrReference {
                                            build_id,
                                            pid,
                                            aslr_reference,
                                        }) = serde_json::from_str::<ClientMsg>(&text)
                                            && let Ok(mut slot) = aslr.lock()
                                        {
                                            *slot = Some(AslrReport {
                                                build_id,
                                                pid,
                                                aslr_reference,
                                            });
                                        }
                                    }
                                    Some(Ok(_)) => {}
                                    _ => break,
                                }
                            }
                        }
                    }
                });
            }
        });

        Ok((Self { tx, aslr }, actual_port))
    }

    /// Broadcast a dev-server message to every connected client.
    pub fn send(&self, msg: &DevserverMsg) {
        if let Ok(text) = serde_json::to_string(msg) {
            let _ = self.tx.send(text);
        }
    }

    /// Broadcast a jump table to be applied by the matching client.
    pub fn hot_reload(
        &self,
        table: JumpTable,
        build_id: u64,
        pid: Option<u32>,
    ) {
        self.send(&DevserverMsg::HotReload(HotReloadMsg {
            templates: Vec::new(),
            assets: Vec::new(),
            jump_table: Some(table),
            ms_elapsed: 0,
            for_build_id: Some(build_id),
            for_pid: pid,
        }));
    }

    /// The latest ASLR reference reported by any client.
    pub fn latest_aslr(&self) -> Option<AslrReport> {
        self.aslr.lock().ok().and_then(|slot| *slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_tungstenite::tungstenite::Message;

    #[tokio::test]
    async fn client_reports_aslr_and_receives_hot_reload() {
        let (server, port) =
            HotPatchServer::start(0).await.expect("start server");
        let url = format!("ws://127.0.0.1:{port}/");
        let (mut ws, _) = tokio_tungstenite::connect_async(&url)
            .await
            .expect("connect");

        // Report an ASLR reference.
        let hello = serde_json::to_string(&ClientMsg::AslrReference {
            build_id: 7,
            pid: Some(1234),
            aslr_reference: 0x1_4000_0000,
        })
        .unwrap();
        ws.send(Message::Text(hello.into())).await.unwrap();

        // The server records it (allow the reader task a moment).
        let mut report = None;
        for _ in 0..50 {
            if let Some(r) = server.latest_aslr() {
                report = Some(r);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert_eq!(
            report,
            Some(AslrReport {
                build_id: 7,
                pid: Some(1234),
                aslr_reference: 0x1_4000_0000,
            })
        );

        // A broadcast reaches the client.
        server.hot_reload(
            JumpTable {
                lib: std::path::PathBuf::from("libpatch.dll"),
                map: Default::default(),
                aslr_reference: 0,
                new_base_address: 0,
                ifunc_count: 0,
            },
            7,
            Some(1234),
        );
        let msg = ws.next().await.expect("message").expect("ok");
        let text = msg.into_text().unwrap();
        assert!(text.contains("\"HotReload\""), "{text}");
        assert!(text.contains("\"for_build_id\":7"), "{text}");
    }

    #[tokio::test]
    async fn client_receives_a_populated_jump_table() {
        let (server, port) =
            HotPatchServer::start(0).await.expect("start server");
        let url = format!("ws://127.0.0.1:{port}/");
        let (mut ws, _) = tokio_tungstenite::connect_async(&url)
            .await
            .expect("connect");

        // A native client reports its pid and runtime base address.
        let hello = serde_json::to_string(&ClientMsg::AslrReference {
            build_id: 42,
            pid: Some(99),
            aslr_reference: 0x7ff6_0000_0000,
        })
        .unwrap();
        ws.send(Message::Text(hello.into())).await.unwrap();

        let mut map = subsecond_types::AddressMap::default();
        map.insert(0x1000, 0x2000);
        map.insert(0x1010, 0x2010);
        server.hot_reload(
            JumpTable {
                lib: std::path::PathBuf::from("libpatch.dll"),
                map,
                aslr_reference: 0x7ff6_0000_0000,
                new_base_address: 0x1000,
                ifunc_count: 0,
            },
            42,
            Some(99),
        );

        // The client receives and the payload round-trips intact.
        let msg = ws.next().await.expect("message").expect("ok");
        let text = msg.into_text().unwrap();
        match serde_json::from_str::<DevserverMsg>(&text).expect("parse") {
            DevserverMsg::HotReload(m) => {
                assert_eq!(m.for_build_id, Some(42));
                assert_eq!(m.for_pid, Some(99));
                let table = m.jump_table.expect("jump table present");
                assert_eq!(table.map.len(), 2);
                assert_eq!(table.map.get(&0x1000), Some(&0x2000));
                assert_eq!(table.map.get(&0x1010), Some(&0x2010));
                assert_eq!(table.aslr_reference, 0x7ff6_0000_0000);
                assert_eq!(table.new_base_address, 0x1000);
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }
}
