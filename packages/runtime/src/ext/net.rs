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

//! Net extension — TCP connect/listen/read/write, resolve.

use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::ops::{self, OpDecl};
use crate::permissions::Permissions;
use crate::resource_table::{Resource, ResourceId, ResourceTable};
use crate::RuntimeExtension;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct TcpStreamHandle {
    pub stream: tokio::net::TcpStream,
}

impl Resource for TcpStreamHandle {
    fn name(&self) -> &str {
        "tcp_stream"
    }
}

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("net")
        .ops(vec![
            OpDecl::new_async_with_input("net.resolve", |_state: ops::SharedOpState, input: serde_json::Value| {
                let host = input.as_str().unwrap_or("").to_string();
                Box::pin(async move {
                    let addrs: Vec<String> = match tokio::net::lookup_host(host).await {
                        Ok(iter) => iter.map(|a| a.to_string()).collect(),
                        Err(_) => vec![],
                    };
                    Ok(serde_json::json!(addrs))
                })
            }),
            OpDecl::new_async_with_input("net.connect_tcp", |state: ops::SharedOpState, input: serde_json::Value| {
                let host = input.get("host").and_then(|v| v.as_str()).unwrap_or("localhost").to_string();
                let port = input.get("port").and_then(|v| v.as_u64()).unwrap_or(0) as u16;
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_net(&host, port)?;
                    }
                    let stream = tokio::net::TcpStream::connect((host.as_str(), port)).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, format!("connect {host}:{port}: {e}"))
                    })?;
                    let mut locked = state.lock().await;
                    let table = locked.get_mut::<ResourceTable>()
                        .ok_or_else(|| RuntimeError::internal("no resource table"))?;
                    let id: ResourceId = table.add(Box::new(TcpStreamHandle { stream }));
                    Ok(serde_json::json!({ "rid": id }))
                })
            }),
            OpDecl::new_async_with_input("net.read", |state: ops::SharedOpState, input: serde_json::Value| {
                let rid = input.as_u64().unwrap_or(0) as ResourceId;
                Box::pin(async move {
                    let mut locked = state.lock().await;
                    let table = locked.get_mut::<ResourceTable>()
                        .ok_or_else(|| RuntimeError::internal("no resource table"))?;
                    let handle = table.get_typed_mut::<TcpStreamHandle>(rid)
                        .ok_or_else(|| RuntimeError::resource("tcp stream not found"))?;
                    let mut buf = vec![0u8; 4096];
                    let n = handle.stream.read(&mut buf).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, e.to_string())
                    })?;
                    buf.truncate(n);
                    Ok(serde_json::json!(String::from_utf8_lossy(&buf).to_string()))
                })
            }),
            OpDecl::new_async_with_input("net.write", |state: ops::SharedOpState, input: serde_json::Value| {
                let rid = input.get("rid").and_then(|v| v.as_u64()).unwrap_or(0) as ResourceId;
                let data = input.get("data").and_then(|v| v.as_str()).unwrap_or("").to_string();
                Box::pin(async move {
                    let mut locked = state.lock().await;
                    let table = locked.get_mut::<ResourceTable>()
                        .ok_or_else(|| RuntimeError::internal("no resource table"))?;
                    let handle = table.get_typed_mut::<TcpStreamHandle>(rid)
                        .ok_or_else(|| RuntimeError::resource("tcp stream not found"))?;
                    handle.stream.write_all(data.as_bytes()).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, e.to_string())
                    })?;
                    Ok(serde_json::json!({ "bytes": data.len() }))
                })
            }),
            OpDecl::new_async_with_input("net.close", |state: ops::SharedOpState, input: serde_json::Value| {
                let rid = input.as_u64().unwrap_or(0) as ResourceId;
                Box::pin(async move {
                    let mut locked = state.lock().await;
                    let table = locked.get_mut::<ResourceTable>()
                        .ok_or_else(|| RuntimeError::internal("no resource table"))?;
                    table.close(rid)?;
                    Ok(serde_json::json!({ "ok": true }))
                })
            }),
        ])
        .build()
}