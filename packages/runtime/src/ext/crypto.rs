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

//! Crypto extension — hash, random bytes, hex encode/decode.

use crate::{
    RuntimeExtension, error::RuntimeError, ops::OpDecl, type_map::OpState,
};
use rand::RngCore;
use sha2::{Digest, Sha256};

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("crypto")
        .ops(vec![
            OpDecl::new_sync_with_input(
                "crypto.sha256",
                |_state: &mut OpState, input: serde_json::Value| {
                    let data = input.as_str().unwrap_or("").as_bytes();
                    let hash = Sha256::digest(data);
                    Ok(serde_json::json!(hex::encode(hash)))
                },
            ),
            OpDecl::new_sync_with_input(
                "crypto.random_bytes",
                |_state: &mut OpState, input: serde_json::Value| {
                    let len = input.as_u64().unwrap_or(32).min(65536) as usize;
                    let mut bytes = vec![0u8; len];
                    rand::thread_rng().fill_bytes(&mut bytes);
                    Ok(serde_json::json!(hex::encode(bytes)))
                },
            ),
            OpDecl::new_sync_with_input(
                "crypto.hex_encode",
                |_state: &mut OpState, input: serde_json::Value| {
                    input
                        .as_str()
                        .map(|s| serde_json::json!(hex::encode(s)))
                        .ok_or_else(|| {
                            RuntimeError::new(
                                crate::error::RuntimeErrorKind::OpExecution,
                                "expected string",
                            )
                        })
                },
            ),
            OpDecl::new_sync_with_input(
                "crypto.hex_decode",
                |_state: &mut OpState, input: serde_json::Value| {
                    let s = input.as_str().unwrap_or("");
                    let bytes = hex::decode(s).map_err(|e| {
                        RuntimeError::new(
                            crate::error::RuntimeErrorKind::OpExecution,
                            e.to_string(),
                        )
                    })?;
                    Ok(serde_json::json!(
                        String::from_utf8_lossy(&bytes).to_string()
                    ))
                },
            ),
        ])
        .build()
}
