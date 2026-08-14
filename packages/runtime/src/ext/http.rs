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

//! HTTP extension — fetch (GET/POST), basic HTTP client.

use crate::{
    RuntimeExtension,
    error::{RuntimeError, RuntimeErrorKind},
    ops::{self, OpDecl},
};

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("http")
        .ops(vec![
            OpDecl::new_async_with_input("http.fetch", |_state: ops::SharedOpState, input: serde_json::Value| {
                let url = input.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let method = input.get("method").and_then(|v| v.as_str()).unwrap_or("GET").to_string();
                let body = input.get("body").and_then(|v| v.as_str()).map(|s| s.to_string());
                Box::pin(async move {
                    let client = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(30))
                        .build()
                        .map_err(|e| RuntimeError::new(RuntimeErrorKind::Internal, e.to_string()))?;
                    let req = {
                        let r = match method.to_uppercase().as_str() {
                            "GET" => client.get(&url),
                            "POST" => {
                                let mut r = client.post(&url);
                                if let Some(b) = &body {
                                    r = r.body(b.clone());
                                }
                                r
                            }
                            "PUT" => {
                                let mut r = client.put(&url);
                                if let Some(b) = &body {
                                    r = r.body(b.clone());
                                }
                                r
                            }
                            "DELETE" => client.delete(&url),
                            _ => client.get(&url),
                        };
                        r
                    };
                    let resp = req.send().await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, format!("http.fetch {url}: {e}"))
                    })?;
                    let status = resp.status().as_u16();
                    let headers = resp.headers().clone();
                    let body_bytes = resp.bytes().await.unwrap_or_default();
                    let header_map: std::collections::HashMap<String, String> = headers
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                        .collect();
                    Ok(serde_json::json!({
                        "status": status,
                        "headers": header_map,
                        "body": String::from_utf8_lossy(&body_bytes).to_string(),
                    }))
                })
            }),
        ])
        .build()
}
