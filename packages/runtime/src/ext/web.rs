//! Web extension — timers (setTimeout, setInterval), btoa/atob.

use crate::error::RuntimeError;
use crate::ops::{self, OpDecl};
use crate::RuntimeExtension;
use tokio::time::{sleep, Duration};

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("web")
        .ops(vec![
            OpDecl::new_sync_with_input("web.btoa", |_state: &mut crate::type_map::OpState, input: serde_json::Value| {
                let s = input.as_str().unwrap_or("");
                let encoded = base64_encode(s.as_bytes());
                Ok(serde_json::json!(encoded))
            }),
            OpDecl::new_sync_with_input("web.atob", |_state: &mut crate::type_map::OpState, input: serde_json::Value| {
                let s = input.as_str().unwrap_or("");
                let decoded = String::from_utf8(base64_decode(s).map_err(|e| {
                    RuntimeError::new(crate::error::RuntimeErrorKind::OpExecution, e.to_string())
                })?)
                .map_err(|e| RuntimeError::new(crate::error::RuntimeErrorKind::OpExecution, e.to_string()))?;
                Ok(serde_json::json!(decoded))
            }),
            OpDecl::new_async_with_input("web.set_timeout", |_state: ops::SharedOpState, input: serde_json::Value| {
                let ms = input.as_u64().unwrap_or(0);
                Box::pin(async move {
                    if ms > 0 {
                        sleep(Duration::from_millis(ms)).await;
                    }
                    Ok(serde_json::json!({ "fired": true }))
                })
            }),
            OpDecl::new_async_with_input("web.sleep", |_state: ops::SharedOpState, input: serde_json::Value| {
                let ms = input.as_u64().unwrap_or(1000);
                Box::pin(async move {
                    sleep(Duration::from_millis(ms)).await;
                    Ok(serde_json::json!({ "slept": ms }))
                })
            }),
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