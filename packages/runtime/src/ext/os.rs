//! OS extension — environment variables, exit, hostname, loadavg, uptime.

use crate::error::RuntimeError;
use crate::ops::OpDecl;
use crate::permissions::Permissions;
use crate::type_map::OpState;
use crate::RuntimeExtension;

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("os")
        .ops(vec![
            OpDecl::new_sync_with_input("os.env_get", |state: &mut OpState, input: serde_json::Value| {
                let key = input.as_str().unwrap_or("").to_string();
                state.get::<Permissions>().ok_or_else(|| RuntimeError::internal("no permissions"))?.check_env(&key)?;
                Ok(serde_json::json!(std::env::var(&key).ok()))
            }),
            OpDecl::new_sync("os.env_vars", |state: &mut OpState| {
                state.get::<Permissions>().ok_or_else(|| RuntimeError::internal("no permissions"))?.check_sys()?;
                let vars: std::collections::HashMap<String, String> = std::env::vars().collect();
                Ok(serde_json::json!(vars))
            }),
            OpDecl::new_sync_with_input("os.exit", |_state: &mut OpState, input: serde_json::Value| {
                let code = input.as_i64().unwrap_or(0) as i32;
                std::process::exit(code);
            }),
            OpDecl::new_sync("os.hostname", |_state: &mut OpState| {
                let hostname = std::env::var("HOSTNAME")
                    .or_else(|_| std::env::var("COMPUTERNAME"))
                    .unwrap_or_else(|_| "unknown".to_string());
                Ok(serde_json::json!(hostname))
            }),
            OpDecl::new_sync("os.uptime", |_state: &mut OpState| {
                Ok(serde_json::json!(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs_f64())
                    .unwrap_or(0.0)))
            }),
            OpDecl::new_sync("os.loadavg", |_state: &mut OpState| {
                #[cfg(target_os = "linux")]
                {
                    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
                        if let Some(parts) = content.split_whitespace().next() {
                            return Ok(serde_json::json!(parts));
                        }
                    }
                }
                Ok(serde_json::json!(null))
            }),
        ])
        .build()
}