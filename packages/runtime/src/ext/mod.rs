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

//! Extension modules for the MontRS runtime.
//!
//! Each module provides a function that returns a `RuntimeExtension` with
//! ops for that domain. Inspired by Deno's `ext/` directory.

pub mod atomic;
pub mod console;
pub mod crypto;
pub mod fs;
#[cfg(feature = "http")]
pub mod http;
pub mod net;
pub mod os;
pub mod process;
pub mod web;

use crate::RuntimeExtension;

/// Load all default extensions (FS, Net, OS, Console, Crypto, Web).
pub fn default_extensions() -> Vec<RuntimeExtension> {
    vec![
        fs::init(),
        net::init(),
        os::init(),
        console::init(),
        crypto::init(),
        web::init(),
    ]
}

/// Load all extensions including heavier ones (HTTP, Process).
#[cfg(feature = "http")]
pub fn full_extensions() -> Vec<RuntimeExtension> {
    let mut exts = default_extensions();
    exts.push(http::init());
    exts.push(process::init());
    exts
}

#[cfg(not(feature = "http"))]
pub fn full_extensions() -> Vec<RuntimeExtension> {
    let mut exts = default_extensions();
    exts.push(process::init());
    exts
}
