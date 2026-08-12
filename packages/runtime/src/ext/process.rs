//! Process extension — run command with output.

use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::ops::{self, OpDecl};
use crate::permissions::Permissions;
use crate::RuntimeExtension;
use tokio::process::Command;

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("process")
        .ops(vec![
            OpDecl::new_async_with_input("process.run", |state: ops::SharedOpState, input: serde_json::Value| {
                let cmd = input.get("cmd").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let args: Vec<String> = input.get("args")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_run(&cmd)?;
                    }
                    #[cfg(unix)]
                    let output = Command::new("sh").arg("-c").arg(&cmd).args(&args).output().await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, format!("process.run {cmd}: {e}"))
                    })?;
                    #[cfg(not(unix))]
                    let output = Command::new("cmd").arg("/C").arg(&cmd).args(&args).output().await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, format!("process.run {cmd}: {e}"))
                    })?;
                    Ok(serde_json::json!({
                        "status": output.status.code(),
                        "success": output.status.success(),
                        "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
                        "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
                    }))
                })
            }),
        ])
        .build()
}