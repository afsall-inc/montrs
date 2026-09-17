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

//! Stable C ABI between the MontRS dev shell and an application dylib.
//!
//! In dev, `montrs serve` builds the application's library as a `cdylib` and a
//! generic shell loads it. On a change the library is rebuilt and a fresh copy
//! is loaded; the shell repoints its vtable and leaks the old library (Windows
//! cannot safely unload). Everything crossing this boundary is plain C types —
//! no Rust types, no framework types — so the two sides can be rebuilt
//! independently.

use core::ffi::c_char;

/// ABI version of the vtable. Bumped on any layout/semantic change; the shell
/// refuses a mismatch.
pub const ABI_VERSION: u32 = 1;

/// A `(pointer, length)` view over UTF-8 bytes owned by the dylib.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MontrsBytes {
    pub ptr: *mut u8,
    pub len: usize,
}

impl MontrsBytes {
    /// An empty byte view.
    pub const EMPTY: MontrsBytes = MontrsBytes {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
}

/// A request passed from the shell to the dylib.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MontrsRequest {
    /// NUL-terminated UTF-8 request path (including query string).
    pub path: *const c_char,
    /// NUL-terminated UTF-8 HTTP method.
    pub method: *const c_char,
}

/// A response produced by the dylib. Both byte views are owned by the dylib and
/// released with [`MontrsAppVtable::free_response`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MontrsResponse {
    /// HTTP status code.
    pub status: u16,
    /// `Content-Type` value (may be empty).
    pub content_type: MontrsBytes,
    /// Response body.
    pub body: MontrsBytes,
}

/// The dylib's entry points.
#[repr(C)]
pub struct MontrsAppVtable {
    /// Must equal [`ABI_VERSION`].
    pub abi_version: u32,
    /// Render `req` into `out`.
    ///
    /// # Safety
    /// `req` must point to a valid [`MontrsRequest`]; `out` must be writable and
    /// zero-initialized.
    pub render: unsafe extern "C" fn(
        req: *const MontrsRequest,
        out: *mut MontrsResponse,
    ),
    /// Release the buffers in a response produced by `render`.
    ///
    /// # Safety
    /// `out` must be a response previously produced by `render`.
    pub free_response: unsafe extern "C" fn(out: *mut MontrsResponse),
}

/// The symbol a MontRS app dylib must export.
pub type MontrsAppEntry = unsafe extern "C" fn() -> *const MontrsAppVtable;

/// Name of the exported entry symbol.
pub const ENTRY_SYMBOL: &[u8] = b"montrs_app_entry\0";

/// Export the stable C entry the dev shell loads from an app `cdylib`.
///
/// Invoke once, at module scope in the app library:
///
/// ```ignore
/// #[cfg(not(target_arch = "wasm32"))]
/// montrs_app_abi::export_app!(app::build_spec(), || view! { <app::Shell /> });
/// ```
///
/// This lives in the (dependency-free) ABI crate on purpose: the app library
/// only needs `montrs-core` and this crate, so its `cdylib` stays light and
/// fast to rebuild. The app is built lazily on first request, so a reloaded
/// library starts from a clean app (see the state caveat in the hot-reload
/// guide).
#[macro_export]
macro_rules! export_app {
    ($spec:expr, $root:expr) => {
        #[cfg(all(not(target_arch = "wasm32"), feature = "ssr"))]
        static __MONTRS_APP: ::std::sync::OnceLock<::montrs_core::serve::SsrApp> =
            ::std::sync::OnceLock::new();

        #[cfg(all(not(target_arch = "wasm32"), feature = "ssr"))]
        unsafe extern "C" fn __montrs_render(
            req: *const $crate::MontrsRequest,
            out: *mut $crate::MontrsResponse,
        ) {
            if req.is_null() || out.is_null() {
                return;
            }
            let req = unsafe { &*req };
            let path = unsafe { ::std::ffi::CStr::from_ptr(req.path) }
                .to_string_lossy()
                .into_owned();
            let method = unsafe { ::std::ffi::CStr::from_ptr(req.method) }
                .to_string_lossy()
                .into_owned();

            let app = __MONTRS_APP.get_or_init(|| {
                let spec = $spec;
                ::montrs_core::serve::SsrApp::build(spec.router, $root)
                    .expect("failed to build MontRS SSR app")
            });

            let (status, content_type, body) =
                match app.render(&method, &path) {
                    Ok((status, headers, body)) => {
                        let ct = headers
                            .iter()
                            .find(|(k, _)| {
                                k.eq_ignore_ascii_case("content-type")
                            })
                            .map(|(_, v)| v.clone())
                            .unwrap_or_default();
                        (status, ct, body)
                    }
                    Err(e) => (
                        500,
                        "text/plain; charset=utf-8".to_string(),
                        format!("montrs render error: {e}").into_bytes(),
                    ),
                };

            let body = body.into_boxed_slice();
            let body_len = body.len();
            let content_type = content_type.into_bytes().into_boxed_slice();
            let ct_len = content_type.len();
            unsafe {
                *out = $crate::MontrsResponse {
                    status,
                    content_type: $crate::MontrsBytes {
                        ptr: ::std::boxed::Box::into_raw(content_type) as *mut u8,
                        len: ct_len,
                    },
                    body: $crate::MontrsBytes {
                        ptr: ::std::boxed::Box::into_raw(body) as *mut u8,
                        len: body_len,
                    },
                };
            }
        }

        #[cfg(all(not(target_arch = "wasm32"), feature = "ssr"))]
        unsafe extern "C" fn __montrs_free_response(
            out: *mut $crate::MontrsResponse,
        ) {
            if out.is_null() {
                return;
            }
            let resp = unsafe { &mut *out };
            for bytes in [resp.content_type, resp.body] {
                if !bytes.ptr.is_null() {
                    let slice =
                        ::std::ptr::slice_from_raw_parts_mut(bytes.ptr, bytes.len);
                    drop(unsafe { ::std::boxed::Box::from_raw(slice) });
                }
            }
            unsafe { *out = ::std::mem::zeroed() };
        }

        #[cfg(all(not(target_arch = "wasm32"), feature = "ssr"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn montrs_app_entry()
        -> *const $crate::MontrsAppVtable {
            static VTABLE: $crate::MontrsAppVtable =
                $crate::MontrsAppVtable {
                    abi_version: $crate::ABI_VERSION,
                    render: __montrs_render,
                    free_response: __montrs_free_response,
                };
            &VTABLE
        }
    };
}
