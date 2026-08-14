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

//! Web extension — timers (setTimeout, setInterval), btoa/atob.

use crate::{
    RuntimeExtension,
    error::RuntimeError,
    ops::{self, OpDecl},
};
use tokio::time::{Duration, sleep};

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("web")
        .ops(vec![
            OpDecl::new_sync_with_input(
                "web.btoa",
                |_state: &mut crate::type_map::OpState,
                 input: serde_json::Value| {
                    let s = input.as_str().unwrap_or("");
                    let encoded = base64_encode(s.as_bytes());
                    Ok(serde_json::json!(encoded))
                },
            ),
            OpDecl::new_sync_with_input(
                "web.atob",
                |_state: &mut crate::type_map::OpState,
                 input: serde_json::Value| {
                    let s = input.as_str().unwrap_or("");
                    let decoded =
                        String::from_utf8(base64_decode(s).map_err(|e| {
                            RuntimeError::new(
                                crate::error::RuntimeErrorKind::OpExecution,
                                e.to_string(),
                            )
                        })?)
                        .map_err(|e| {
                            RuntimeError::new(
                                crate::error::RuntimeErrorKind::OpExecution,
                                e.to_string(),
                            )
                        })?;
                    Ok(serde_json::json!(decoded))
                },
            ),
            OpDecl::new_async_with_input(
                "web.set_timeout",
                |_state: ops::SharedOpState, input: serde_json::Value| {
                    let ms = input.as_u64().unwrap_or(0);
                    Box::pin(async move {
                        if ms > 0 {
                            sleep(Duration::from_millis(ms)).await;
                        }
                        Ok(serde_json::json!({ "fired": true }))
                    })
                },
            ),
            OpDecl::new_async_with_input(
                "web.sleep",
                |_state: ops::SharedOpState, input: serde_json::Value| {
                    let ms = input.as_u64().unwrap_or(1000);
                    Box::pin(async move {
                        sleep(Duration::from_millis(ms)).await;
                        Ok(serde_json::json!({ "slept": ms }))
                    })
                },
            ),
        ])
        .build()
}

fn base64_encode(input: &[u8]) -> String {
    use base64::Engine as _;
    let engine = base64::engine::general_purpose::STANDARD;
    engine.encode(input)
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    use base64::Engine as _;
    let engine = base64::engine::general_purpose::STANDARD;
    engine.decode(input).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btoa() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
    }

    #[test]
    fn test_atob() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
    }
}
