//! Crypto extension — hash, random bytes, hex encode/decode.

use crate::error::RuntimeError;
use crate::ops::OpDecl;
use crate::type_map::OpState;
use crate::RuntimeExtension;
use sha2::{Digest, Sha256};
use rand::RngCore;

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("crypto")
        .ops(vec![
            OpDecl::new_sync_with_input("crypto.sha256", |_state: &mut OpState, input: serde_json::Value| {
                let data = input.as_str().unwrap_or("").as_bytes();
                let hash = Sha256::digest(data);
                Ok(serde_json::json!(hex::encode(hash)))
            }),
            OpDecl::new_sync_with_input("crypto.random_bytes", |_state: &mut OpState, input: serde_json::Value| {
                let len = input.as_u64().unwrap_or(32).min(65536) as usize;
                let mut bytes = vec![0u8; len];
                rand::thread_rng().fill_bytes(&mut bytes);
                Ok(serde_json::json!(hex::encode(bytes)))
            }),
            OpDecl::new_sync_with_input("crypto.hex_encode", |_state: &mut OpState, input: serde_json::Value| {
                input.as_str().map(|s| serde_json::json!(hex::encode(s)))
                    .ok_or_else(|| RuntimeError::new(crate::error::RuntimeErrorKind::OpExecution, "expected string"))
            }),
            OpDecl::new_sync_with_input("crypto.hex_decode", |_state: &mut OpState, input: serde_json::Value| {
                let s = input.as_str().unwrap_or("");
                let bytes = hex::decode(s).map_err(|e| RuntimeError::new(crate::error::RuntimeErrorKind::OpExecution, e.to_string()))?;
                Ok(serde_json::json!(String::from_utf8_lossy(&bytes).to_string()))
            }),
        ])
        .build()
}