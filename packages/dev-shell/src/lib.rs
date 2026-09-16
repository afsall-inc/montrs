// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generic dev shell: owns the HTTP listener and hosts a hot-swappable app
//! dylib. On a change the CLI rebuilds the dylib and POSTs its new path to the
//! control endpoint; the shell loads it and atomically swaps the vtable, so the
//! listener is never dropped. Old libraries are leaked (Windows cannot safely
//! unload), bounded by the number of reloads in a dev session.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use anyhow::{Result, anyhow};
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use libloading::Library;
use montrs_app_abi::{
    ABI_VERSION, ENTRY_SYMBOL, MontrsAppEntry, MontrsAppVtable, MontrsRequest,
    MontrsResponse,
};

/// A loaded app dylib and its vtable.
pub struct LoadedApp {
    // Kept alive for the lifetime of the shell; intentionally never unloaded.
    _lib: Library,
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
                return Err(anyhow!("{} returned a null vtable", path.display()));
            }
            if (*vtable).abi_version != ABI_VERSION {
                return Err(anyhow!(
                    "ABI mismatch: app {} vs shell {}",
                    (*vtable).abi_version,
                    ABI_VERSION
                ));
            }
            Ok(Self { _lib: lib, vtable })
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
                String::from_utf8_lossy(&bytes(&resp.content_type)).into_owned();
            let status = resp.status;

            ((*self.vtable).free_response)(&mut resp);
            Ok((status, content_type, body))
        }
    }
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
pub async fn serve(addr: &str, dylib: &Path) -> Result<()> {
    let app: SharedApp = Arc::new(RwLock::new(Arc::new(
        LoadedApp::load(dylib)?,
    )));

    let router = axum::Router::new()
        .route(
            "/__montrs/reload",
            axum::routing::post(reload_handler),
        )
        .fallback(axum::routing::any(app_handler))
        .with_state(app);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("montrs-dev-shell listening on http://{addr}");
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

async fn reload_handler(
    State(app): State<SharedApp>,
    axum::Json(req): axum::Json<ReloadRequest>,
) -> Response {
    match LoadedApp::load(&req.path) {
        Ok(loaded) => {
            *app.write().unwrap() = Arc::new(loaded);
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

    let rendered = tokio::task::spawn_blocking(move || {
        current.render(&method, &uri)
    })
    .await;

    match rendered {
        Ok(Ok((status, content_type, body))) => {
            let mut response = Response::builder()
                .status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));
            if !content_type.is_empty() {
                response = response.header(axum::http::header::CONTENT_TYPE, content_type);
            }
            response
                .body(Body::from(body))
                .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
        Ok(Err(e)) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("render error: {e}"))
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("render task failed: {e}"),
        )
            .into_response(),
    }
}
