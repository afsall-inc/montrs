// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Native hot-patch client.
//!
//! Runs inside the application process (the "fat" binary), connects to the dev
//! server's hot-patch socket, reports the process's runtime base address, and
//! applies the jump tables it receives via [`subsecond::apply_patch`]. After a
//! patch is applied, any code routed through a `subsecond::call` cutover uses
//! the new version without restarting.

use crate::{ClientMsg, DevserverMsg};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

/// Connect to `ws://{addr}/` on a background thread and keep this process
/// patched for as long as it runs. Safe to call once at startup; a missing
/// server is retried.
pub fn connect(addr: &str, build_id: u64) {
    let url = format!("ws://{addr}/");
    std::thread::spawn(move || {
        let Ok(rt) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        else {
            return;
        };
        rt.block_on(async move {
            let our_pid = std::process::id();
            let probe = std::env::var_os("MONTRS_HOTPATCH_PROBE").is_some();
            loop {
                if let Ok((ws, _)) =
                    tokio_tungstenite::connect_async(&url).await
                {
                    let (mut sink, mut source) = ws.split();
                    if probe {
                        eprintln!(
                            "[montrs-hotpatch] connected to {url} (pid \
                             {our_pid})"
                        );
                    }

                    // Report the runtime base address so the patch's stubs can
                    // resolve against the running image.
                    let hello = ClientMsg::AslrReference {
                        build_id,
                        pid: Some(our_pid),
                        aslr_reference: subsecond::aslr_reference() as u64,
                    };
                    if let Ok(text) = serde_json::to_string(&hello) {
                        let _ = sink.send(Message::Text(text.into())).await;
                    }

                    while let Some(Ok(msg)) = source.next().await {
                        let Message::Text(text) = msg else {
                            continue;
                        };
                        let Ok(DevserverMsg::HotReload(m)) =
                            serde_json::from_str::<DevserverMsg>(&text)
                        else {
                            continue;
                        };
                        let Some(table) = m.jump_table else {
                            continue;
                        };
                        if m.for_pid == Some(our_pid) {
                            let entries = table.map.len();
                            let table_aslr = table.aslr_reference;
                            let key = crate::cutover_key();
                            let aslr = subsecond::aslr_reference() as u64;
                            let slide = aslr.wrapping_sub(table_aslr);
                            let expected_fat = key.wrapping_sub(slide);
                            let pre = key != 0
                                && table.map.contains_key(&expected_fat);
                            // SAFETY: the dev server only sends well-formed
                            // tables built against this exact binary (matched
                            // by pid).
                            let result =
                                unsafe { subsecond::apply_patch(table) };
                            let post = unsafe { subsecond::get_jump_table() }
                                .map(|t| key != 0 && t.map.contains_key(&key))
                                .unwrap_or(false);
                            if probe {
                                eprintln!(
                                    "[montrs-hotpatch] applied ({entries}): \
                                     {result:?}\n  key={key:#x} \
                                     aslr={aslr:#x} \
                                     table.aslr={table_aslr:#x} \
                                     slide={slide:#x}\n  \
                                     expected_fat={expected_fat:#x} pre={pre} \
                                     post={post}"
                                );
                            }
                        }
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });
    });
}
