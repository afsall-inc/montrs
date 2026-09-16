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

//! Dev hot-patch runtime.
//!
//! Application authors never touch this crate directly: [`serve!`] wires up a
//! hot-patch cutover and the native patch client, so an app just renders with
//! `montrs_hotpatch::serve!(router, || view! { <Shell /> })` and gets Rust
//! hot-patching for free when the dev server exposes it. In release builds the
//! cutover is a no-op.
//!
//! The macro expands `subsecond::call` *in the calling crate* on purpose: the
//! cutover's monomorphized code has to land in the tip crate's object files for
//! the patch to contain it. Hiding it behind a helper function here would put
//! that code in this crate and the jump-table lookup would never hit.

pub use montrs_dev_hotpatch;
pub use subsecond;

/// The runtime address `subsecond` will key the cutover on. Used by the
/// `MONTRS_HOTPATCH_PROBE` diagnostics.
pub fn cutover_key<O>(f: impl FnMut() -> O) -> u64 {
    subsecond::HotFn::current(f).ptr_address().0
}

/// Route a render through the `subsecond` hot-patch cutover.
///
/// `subsecond::call` returns the closure result unchanged in release builds, so
/// this is free outside the dev server. Prefer the [`serve!`] macro, which
/// inlines the call into the app crate.
#[inline(always)]
pub fn cutover<O>(f: impl FnMut() -> O) -> O {
    subsecond::call(f)
}

/// Connect to the dev server's hot-patch socket when one is exposed.
///
/// Set by `montrs serve` in hot-patch mode; a no-op otherwise.
pub fn install_client_from_env() {
    if let Ok(addr) = std::env::var("MONTRS_HOTPATCH_ADDR") {
        montrs_dev_hotpatch::client::connect(&addr, 0);
    }
}

/// Serve an app's router with the root render wrapped in a hot-patch cutover.
///
/// ```ignore
/// montrs_hotpatch::serve!(website::build_spec().router, || view! { <Shell /> });
/// ```
#[macro_export]
macro_rules! serve {
    ($router:expr, $root:expr) => {{
        $crate::install_client_from_env();
        ::montrs_core::serve::montrs_serve($router, move || {
            if ::std::env::var_os("MONTRS_HOTPATCH_PROBE").is_some() {
                let key = $crate::cutover_key($root.clone());
                $crate::montrs_dev_hotpatch::set_cutover_key(key);
            }
            $crate::subsecond::call($root)
        })
    }};
}

