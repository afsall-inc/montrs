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