// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generic dev shell: owns the HTTP listener and hosts a hot-swappable app
//! dylib. On a change the CLI rebuilds the dylib and POSTs its new path to the
//! control endpoint; the shell loads it and atomically swaps the vtable, so the
//! listener is never dropped. Old libraries are leaked (Windows cannot safely
//! unload), bounded by the number of reloads in a dev session.

use anyhow::{Result, anyhow};
use axum::{
    body::Body,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use libloading::Library;
use montrs_app_abi::{
    ABI_VERSION, ENTRY_SYMBOL, MontrsAppEntry, MontrsAppVtable, MontrsRequest,
    MontrsResponse,
};
use std::{
    mem::ManuallyDrop,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

/// A loaded app dylib and its vtable.
pub struct LoadedApp {
    // Kept alive for the lifetime of the shell; intentionally never unloaded.
    _lib: ManuallyDrop<Library>,
    vtable: *const MontrsAppVtable,
}

// The vtable points at code owned by `_lib`, which lives as long as `self`.
unsafe impl Send for LoadedApp {}
unsafe impl Sync for LoadedApp {}

impl LoadedApp {
    /// Load an app dylib and read its entry.
    pub fn load(path: &Path) -> Result<Self> {
        unsafe {
            let lib = Library::new(path).map_err(|e| {
                anyhow!("failed to load {}: {e}", path.display())
            })?;
            let entry: libloading::Symbol<'_, MontrsAppEntry> =
                lib.get(ENTRY_SYMBOL).map_err(|e| {
                    anyhow!("{} has no montrs_app_entry: {e}", path.display())
                })?;
            let vtable = entry();
            if vtable.is_null() {
                return Err(anyhow!(
                    "{} returned a null vtable",
                    path.display()
                ));
            }
            if (*vtable).abi_version != ABI_VERSION {
                return Err(anyhow!(
                    "ABI mismatch: app {} vs shell {}",
                    (*vtable).abi_version,
                    ABI_VERSION
                ));
            }
            Ok(Self {
                _lib: ManuallyDrop::new(lib),
                vtable,
            })
        }
    }

    /// Render a request to `(status, content_type, body)`.
    pub fn render(
        &self,
        method: &str,
        path: &str,
    ) -> Result<(u16, String, Vec<u8>)> {
        let c_path = std::ffi::CString::new(path)?;
        let c_method = std::ffi::CString::new(method)?;
        unsafe {
            let req = MontrsRequest {
                path: c_path.as_ptr(),
                method: c_method.as_ptr(),
            };
            let mut resp: MontrsResponse = std::mem::zeroed();
            ((*self.vtable).render)(&req, &mut resp);

            let body = bytes(&resp.body);
            let content_type =
                String::from_utf8_lossy(&bytes(&resp.content_type))
                    .into_owned();
            let status = resp.status;

            ((*self.vtable).free_response)(&mut resp);
            Ok((status, content_type, body))
        }
    }

    /// Serialize the app's long-lived state (empty if the app has none).
    pub fn export_state(&self) -> Vec<u8> {
        unsafe {
            let mut out = montrs_app_abi::MontrsBytes {
                ptr: std::ptr::null_mut(),
                len: 0,
            };
            ((*self.vtable).export_state)(&mut out);
            let data = bytes(&out);
            ((*self.vtable).free_state)(out);
            data
        }
    }

    /// Restore state produced by [`LoadedApp::export_state`].
    pub fn import_state(&self, data: &[u8]) {
        unsafe {
            let b = montrs_app_abi::MontrsBytes {
                ptr: data.as_ptr() as *mut u8,
                len: data.len(),
            };
            ((*self.vtable).import_state)(b);
        }
    }
}

/// Load `path` and swap it in, carrying the outgoing app's state across so
/// counters, caches, and in-memory stores survive the reload.
fn swap(app: &SharedApp, path: &Path) -> Result<()> {
    let state = app.read().unwrap().export_state();
    let loaded = LoadedApp::load(path)?;
    if !state.is_empty() {
        loaded.import_state(&state);
    }
    *app.write().unwrap() = Arc::new(loaded);
    Ok(())
}

unsafe fn bytes(b: &montrs_app_abi::MontrsBytes) -> Vec<u8> {
    if b.ptr.is_null() || b.len == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(b.ptr, b.len).to_vec() }
    }
}

/// The current app, swappable at runtime.
pub type SharedApp = Arc<RwLock<Arc<LoadedApp>>>;

#[derive(serde::Deserialize)]
struct ReloadRequest {
    path: PathBuf,
}

/// Serve the app on `addr`, exposing `POST /__montrs/reload` to swap the dylib.
///
/// If `MONTRS_RELOAD_FILE` is set, that file is polled and its contents are
/// treated as the path of the dylib to load; writing a new path reloads the app.
/// This is how the CLI triggers a swap without an HTTP client.
pub async fn serve(addr: &str, dylib: &Path) -> Result<()> {
    let app: SharedApp =
        Arc::new(RwLock::new(Arc::new(LoadedApp::load(dylib)?)));

    if let Ok(reload_file) = std::env::var("MONTRS_RELOAD_FILE") {
        spawn_file_watcher(app.clone(), PathBuf::from(reload_file));
    }

    let router = hotpatch_router(std::env::var("MONTRS_HOTPATCH_ADDR").ok())
        .route("/__montrs/reload", axum::routing::post(reload_handler))
        .fallback(axum::routing::any(app_handler))
        .with_state(app);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("montrs-dev-shell listening on http://{addr}");
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

/// Poll a file whose contents name the current app dylib; reload on change.
fn spawn_file_watcher(app: SharedApp, file: PathBuf) {
    tokio::spawn(async move {
        // Seed with the file's current contents so we do not reload immediately.
        let mut last =
            tokio::fs::read_to_string(&file).await.unwrap_or_default();
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            let Ok(next) = tokio::fs::read_to_string(&file).await else {
                continue;
            };
            let next = next.trim().to_string();
            if next.is_empty() || next == last {
                continue;
            }
            last = next.clone();
            match swap(&app, Path::new(&next)) {
                Ok(()) => tracing::info!("reloaded app dylib {next}"),
                Err(e) => tracing::error!("reload of {next} failed: {e}"),
            }
        }
    });
}

fn hotpatch_router<S>(endpoint: Option<String>) -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().route(
        "/_dioxus",
        axum::routing::get(move |upgrade: WebSocketUpgrade| async move {
            upgrade.on_upgrade(move |client| hotpatch_socket(client, endpoint))
        }),
    )
}

async fn hotpatch_socket(mut client: WebSocket, endpoint: Option<String>) {
    use tokio_tungstenite::tungstenite::Message as UpstreamMessage;

    let Some(addr) = endpoint else {
        let _ = client.send(Message::Close(None)).await;
        return;
    };
    let Ok((mut upstream, _)) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/")).await
    else {
        let _ = client.send(Message::Close(None)).await;
        return;
    };

    loop {
        tokio::select! {
            message = client.recv() => {
                let Some(Ok(message)) = message else {
                    let _ = upstream.close(None).await;
                    break;
                };
                let closing = matches!(message, Message::Close(_));
                let message = match message {
                    Message::Text(text) => UpstreamMessage::Text(text.to_string().into()),
                    Message::Binary(data) => UpstreamMessage::Binary(data),
                    Message::Ping(data) => UpstreamMessage::Ping(data),
                    Message::Pong(data) => UpstreamMessage::Pong(data),
                    Message::Close(frame) => UpstreamMessage::Close(frame.map(|frame| {
                        tokio_tungstenite::tungstenite::protocol::CloseFrame {
                            code: frame.code.into(),
                            reason: frame.reason.to_string().into(),
                        }
                    })),
                };
                if upstream.send(message).await.is_err() || closing {
                    let _ = client.flush().await;
                    break;
                }
            }
            message = upstream.next() => {
                let Some(Ok(message)) = message else {
                    let _ = client.send(Message::Close(None)).await;
                    break;
                };
                let closing = matches!(message, UpstreamMessage::Close(_));
                let message = match message {
                    UpstreamMessage::Text(text) => Message::Text(text.to_string().into()),
                    UpstreamMessage::Binary(data) => Message::Binary(data),
                    UpstreamMessage::Ping(data) => Message::Ping(data),
                    UpstreamMessage::Pong(data) => Message::Pong(data),
                    UpstreamMessage::Close(frame) => Message::Close(frame.map(|frame| {
                        axum::extract::ws::CloseFrame {
                            code: frame.code.into(),
                            reason: frame.reason.to_string().into(),
                        }
                    })),
                    UpstreamMessage::Frame(_) => continue,
                };
                if client.send(message).await.is_err() || closing {
                    let _ = upstream.flush().await;
                    break;
                }
            }
        }
    }
}

async fn reload_handler(
    State(app): State<SharedApp>,
    axum::Json(req): axum::Json<ReloadRequest>,
) -> Response {
    match swap(&app, &req.path) {
        Ok(()) => {
            tracing::info!("reloaded app dylib {}", req.path.display());
            StatusCode::OK.into_response()
        }
        Err(e) => {
            tracing::error!("reload failed: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")).into_response()
        }
    }
}

async fn app_handler(
    State(app): State<SharedApp>,
    req: Request<Body>,
) -> Response {
    let method = req.method().as_str().to_string();
    let uri = req.uri().to_string();
    let current = app.read().unwrap().clone();

    let rendered =
        tokio::task::spawn_blocking(move || current.render(&method, &uri))
            .await;

    match rendered {
        Ok(Ok((status, content_type, body))) => {
            let mut response = Response::builder()
                .status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));
            if !content_type.is_empty() {
                response = response
                    .header(axum::http::header::CONTENT_TYPE, content_type);
            }
            response.body(Body::from(body)).unwrap_or_else(|_| {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            })
        }
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("render error: {e}"),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("render task failed: {e}"),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::{net::TcpListener, task::JoinSet, time::timeout};
    use tokio_tungstenite::{
        accept_async, connect_async,
        tungstenite::{
            Message as UpstreamMessage,
            protocol::{CloseFrame, frame::coding::CloseCode},
        },
    };

    async fn spawn_transport(
        endpoint: Option<String>,
        tasks: &mut JoinSet<()>,
    ) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tasks.spawn(async move {
            axum::serve(listener, hotpatch_router::<()>(endpoint))
                .await
                .unwrap();
        });
        format!("ws://{addr}/_dioxus")
    }

    #[tokio::test]
    async fn hotpatch_transport_roundtrip_and_close() {
        timeout(Duration::from_secs(5), async {
            for upstream_closes in [false, true] {
                let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                let addr = listener.local_addr().unwrap();
                let mut tasks = JoinSet::new();
                let url =
                    spawn_transport(Some(addr.to_string()), &mut tasks).await;
                let close = Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: "transport complete".into(),
                });
                let upstream_close = close.clone();
                tasks.spawn(async move {
                    let (stream, _) = listener.accept().await.unwrap();
                    let mut upstream = accept_async(stream).await.unwrap();
                    for expected in [
                        UpstreamMessage::Text("transport only".into()),
                        UpstreamMessage::Binary(vec![0, 1, 255].into()),
                    ] {
                        let message = upstream.next().await.unwrap().unwrap();
                        assert_eq!(message, expected);
                        upstream.send(message).await.unwrap();
                    }
                    if upstream_closes {
                        upstream.close(upstream_close.clone()).await.unwrap();
                    }
                    assert_eq!(
                        upstream.next().await.unwrap().unwrap(),
                        UpstreamMessage::Close(upstream_close),
                    );
                    if !upstream_closes {
                        let _ = upstream.flush().await;
                    }
                });
                let (mut client, response) = connect_async(url).await.unwrap();
                assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
                for message in [
                    UpstreamMessage::Text("transport only".into()),
                    UpstreamMessage::Binary(vec![0, 1, 255].into()),
                ] {
                    client.send(message.clone()).await.unwrap();
                    assert_eq!(client.next().await.unwrap().unwrap(), message);
                }
                if !upstream_closes {
                    client.close(close.clone()).await.unwrap();
                }
                assert_eq!(
                    client.next().await.unwrap().unwrap(),
                    UpstreamMessage::Close(close),
                );
                if upstream_closes {
                    let _ = client.flush().await;
                }
                tasks.join_next().await.unwrap().unwrap();
                tasks.shutdown().await;
            }
        })
        .await
        .expect("hotpatch transport timed out");
    }

    #[tokio::test]
    async fn hotpatch_transport_missing_hub_closes_gracefully() {
        timeout(Duration::from_secs(5), async {
            let mut tasks = JoinSet::new();
            let url = spawn_transport(None, &mut tasks).await;
            let (mut client, response) = connect_async(url).await.unwrap();
            assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
            assert_eq!(
                client.next().await.unwrap().unwrap(),
                UpstreamMessage::Close(None),
            );
            tasks.shutdown().await;
        })
        .await
        .expect("missing hotpatch hub timed out");
    }

    #[tokio::test]
    async fn hotpatch_transport_failed_hub_closes_gracefully() {
        timeout(Duration::from_secs(5), async {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            let mut tasks = JoinSet::new();
            let url = spawn_transport(Some(addr.to_string()), &mut tasks).await;
            tasks.spawn(async move {
                let (stream, _) = listener.accept().await.unwrap();
                drop(stream);
            });
            let (mut client, response) = connect_async(url).await.unwrap();
            assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
            assert_eq!(
                client.next().await.unwrap().unwrap(),
                UpstreamMessage::Close(None),
            );
            tasks.join_next().await.unwrap().unwrap();
            tasks.shutdown().await;
        })
        .await
        .expect("failed hotpatch hub timed out");
    }
}
