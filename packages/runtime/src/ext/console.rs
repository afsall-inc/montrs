//! Console extension — print/log ops.

use crate::ops::OpDecl;
use crate::type_map::OpState;
use crate::RuntimeExtension;

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("console")
        .ops(vec![
            OpDecl::new_sync_with_input("console.log", |_state: &mut OpState, input: serde_json::Value| {
                println!("[console.log] {}", input);
                Ok(serde_json::json!({}))
            }),
            OpDecl::new_sync_with_input("console.info", |_state: &mut OpState, input: serde_json::Value| {
                println!("[console.info] {}", input);
                Ok(serde_json::json!({}))
            }),
            OpDecl::new_sync_with_input("console.warn", |_state: &mut OpState, input: serde_json::Value| {
                eprintln!("[console.warn] {}", input);
                Ok(serde_json::json!({}))
            }),
            OpDecl::new_sync_with_input("console.error", |_state: &mut OpState, input: serde_json::Value| {
                eprintln!("[console.error] {}", input);
                Ok(serde_json::json!({}))
            }),
        ])
        .build()
}