//! HTTP extension — fetch (GET/POST), basic HTTP client.

use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::ops::{self, OpDecl};
use crate::RuntimeExtension;

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