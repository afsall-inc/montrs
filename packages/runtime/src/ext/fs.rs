//! FS extension — file read/write/stat/list/open ops.

use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::ops::{self, OpDecl};
use crate::permissions::Permissions;
use crate::resource_table::{Resource, ResourceId, ResourceTable};
use crate::RuntimeExtension;

/// A file handle resource.
pub struct FileHandle {
    pub path: String,
    pub file: tokio::fs::File,
}

impl Resource for FileHandle {
    fn name(&self) -> &str {
        "file"
    }
}

fn extract_path(input: &serde_json::Value) -> String {
    input.as_str()
        .map(|s| s.to_string())
        .or_else(|| input.get("path").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .unwrap_or_default()
}

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("fs")
        .ops(vec![
            OpDecl::new_async_with_input("fs.read_file", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = extract_path(&input);
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_read(&path)?;
                    }
                    let contents = tokio::fs::read(&path).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.read_file {path}: {e}"))
                    })?;
                    Ok(serde_json::json!(String::from_utf8_lossy(&contents).to_string()))
                })
            }),
            OpDecl::new_async_with_input("fs.write_file", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = input.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let contents = input.get("contents").and_then(|v| v.as_str()).unwrap_or("").to_string();
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_write(&path)?;
                    }
                    tokio::fs::write(&path, &contents).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.write_file {path}: {e}"))
                    })?;
                    Ok(serde_json::json!({ "ok": true, "bytes": contents.len() }))
                })
            }),
            OpDecl::new_async_with_input("fs.stat", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = extract_path(&input);
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_read(&path)?;
                    }
                    let meta = tokio::fs::metadata(&path).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.stat {path}: {e}"))
                    })?;
                    Ok(serde_json::json!({ "path": path, "len": meta.len(), "is_dir": meta.is_dir(), "is_file": meta.is_file() }))
                })
            }),
            OpDecl::new_async_with_input("fs.mkdir", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = extract_path(&input);
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_write(&path)?;
                    }
                    tokio::fs::create_dir_all(&path).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.mkdir {path}: {e}"))
                    })?;
                    Ok(serde_json::json!({ "ok": true }))
                })
            }),
            OpDecl::new_async_with_input("fs.remove", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = extract_path(&input);
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_write(&path)?;
                    }
                    let meta = tokio::fs::metadata(&path).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.remove {path}: {e}"))
                    })?;
                    if meta.is_dir() {
                        tokio::fs::remove_dir_all(&path).await.map_err(|e| {
                            RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.remove {path}: {e}"))
                        })?;
                    } else {
                        tokio::fs::remove_file(&path).await.map_err(|e| {
                            RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.remove {path}: {e}"))
                        })?;
                    }
                    Ok(serde_json::json!({ "ok": true }))
                })
            }),
            OpDecl::new_async_with_input("fs.read_dir", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = extract_path(&input);
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_read(&path)?;
                    }
                    let mut entries = Vec::new();
                    let mut rd = tokio::fs::read_dir(&path).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, format!("fs.read_dir {path}: {e}"))
                    })?;
                    while let Some(entry) = rd.next_entry().await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::ModuleLoad, e.to_string())
                    })? {
                        let meta = entry.metadata().await.ok();
                        entries.push(serde_json::json!({
                            "name": entry.file_name().to_string_lossy(),
                            "is_dir": meta.as_ref().map(|m| m.is_dir()).unwrap_or(false),
                        }));
                    }
                    Ok(serde_json::json!({ "entries": entries }))
                })
            }),
            OpDecl::new_async_with_input("fs.open", |state: ops::SharedOpState, input: serde_json::Value| {
                let path = extract_path(&input);
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_fs_read(&path)?;
                    }
                    let file = tokio::fs::File::open(&path).await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::Resource, format!("open {path}: {e}"))
                    })?;
                    let mut locked = state.lock().await;
                    let table = locked.get_mut::<ResourceTable>()
                        .ok_or_else(|| RuntimeError::internal("no resource table"))?;
                    let id: ResourceId = table.add(Box::new(FileHandle { path: path.clone(), file }));
                    Ok(serde_json::json!({ "rid": id, "path": path }))
                })
            }),
            OpDecl::new_async_with_input("fs.close", |state: ops::SharedOpState, input: serde_json::Value| {
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