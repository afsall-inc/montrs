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

//! Invariant tests for montrs-runtime.

use montrs_runtime::error::{RuntimeError, RuntimeErrorKind};
use montrs_runtime::prelude::*;
use montrs_runtime::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[test]
fn test_arena_alloc() {
    let arena = Arena::new(1024);
    let (_ptr, size) = arena.alloc(100).expect("alloc");
    assert_eq!(size, 104);
    assert!(arena.used() >= 104);
    assert!(arena.remaining() < 1024);
}

#[test]
fn test_arena_reset() {
    let arena = Arena::new(512);
    arena.alloc(100).unwrap();
    assert!(arena.used() > 0);
    arena.reset();
    assert_eq!(arena.used(), 0);
}

#[test]
fn test_arena_overflow() {
    let arena = Arena::new(64);
    assert!(arena.alloc(100).is_none());
}

#[test]
fn test_arena_overflow_does_not_corrupt() {
    // B9 fix: overflow should not advance the cursor.
    let arena = Arena::new(64);
    // Request more than capacity (aligned size > 64).
    assert!(arena.alloc(65).is_none());
    assert_eq!(arena.used(), 0);
    assert!(arena.alloc(32).is_some());
    assert_eq!(arena.used(), 32);
}

#[test]
fn test_tagged_value_int() {
    let v = TaggedValue::from_int(42);
    assert!(v.is_int());
    assert_eq!(v.as_int(), Some(42));
}

#[test]
fn test_tagged_value_float() {
    let v = TaggedValue::from_float(3.14);
    assert!(v.is_float());
    assert!((v.as_float().unwrap() - 3.14).abs() < 1e-9);
}

#[test]
fn test_tagged_value_nan() {
    // B10 fix: NaN should be recognized as float, not int/bool.
    let v = TaggedValue::from_float(f64::NAN);
    assert!(v.is_float());
    assert!(v.as_float().unwrap().is_nan());
    assert!(!v.is_int());
    assert!(!v.is_bool());
}

#[test]
fn test_tagged_value_bool() {
    let v = TaggedValue::from_bool(true);
    assert!(v.is_bool());
    assert_eq!(v.as_bool(), Some(true));
    let v = TaggedValue::from_bool(false);
    assert_eq!(v.as_bool(), Some(false));
}

#[test]
fn test_tagged_value_null() {
    let v = TaggedValue::null();
    assert!(v.is_null());
}

#[test]
fn test_bit_field() {
    let mut bf = BitField::new();
    bf.set(0, 4, 10);
    bf.set(4, 4, 5);
    assert_eq!(bf.get(0, 4), 10);
    assert_eq!(bf.get(4, 4), 5);
}

#[test]
fn test_type_map() {
    let mut map = TypeMap::new();
    map.put(42u32);
    map.put("hello".to_string());
    assert_eq!(map.get::<u32>(), Some(&42));
    assert_eq!(map.get::<String>(), Some(&"hello".to_string()));
    assert!(map.contains::<u32>());
    assert!(!map.contains::<bool>());
    assert_eq!(map.len(), 2);
    let taken: Option<u32> = map.take();
    assert_eq!(taken, Some(42));
    assert!(!map.contains::<u32>());
}

#[test]
fn test_resource_table() {
    use montrs_runtime::resource_table::Resource;
    struct MyResource;
    impl Resource for MyResource {
        fn name(&self) -> &str {
            "my_resource"
        }
    }
    let mut table = ResourceTable::new();
    let id = table.add(Box::new(MyResource));
    assert_eq!(table.len(), 1);
    let res = table.get(id).expect("resource");
    assert_eq!(res.name(), "my_resource");
    table.close(id).unwrap();
    assert_eq!(table.len(), 0);
}

#[test]
fn test_resource_close_result() {
    // B12 fix: close() returns Result, not void.
    use montrs_runtime::resource_table::Resource;
    struct Closeable;
    impl Resource for Closeable {
        fn name(&self) -> &str {
            "closeable"
        }
        fn close(&self) -> Result<(), RuntimeError> {
            Ok(())
        }
    }
    let mut table = ResourceTable::new();
    let id = table.add(Box::new(Closeable));
    assert!(table.close(id).is_ok());
}

#[test]
fn test_op_decl_sync() {
    let mut state = OpState::new();
    let op = OpDecl::new_sync("test.sync", |_state: &mut OpState| {
        Ok(serde_json::json!({ "value": 42 }))
    });
    let result = op.execute(&mut state, None).unwrap();
    assert_eq!(result["value"], 42);
}

#[test]
fn test_op_decl_sync_with_input() {
    let mut state = OpState::new();
    let op = OpDecl::new_sync_with_input(
        "test.input",
        |_state: &mut OpState, input: serde_json::Value| {
            Ok(serde_json::json!({ "echo": input }))
        },
    );
    let result = op
        .execute(&mut state, Some(serde_json::json!("hi")))
        .unwrap();
    assert_eq!(result["echo"], "hi");
}

#[test]
fn test_op_id_uniqueness() {
    // B1 fix: all constructors share a single global counter.
    let op1 = OpDecl::new_sync("a", |_s: &mut OpState| Ok(serde_json::json!({})));
    let op2 = OpDecl::new_async("b", |_s: montrs_runtime::ops::SharedOpState| {
        Box::pin(async { Ok(serde_json::json!({})) })
    });
    let op3 = OpDecl::new_sync_with_input("c", |_s: &mut OpState, _i: serde_json::Value| {
        Ok(serde_json::json!({}))
    });
    let op4 = OpDecl::new_async_with_input("d", |_s: montrs_runtime::ops::SharedOpState, _i: serde_json::Value| {
        Box::pin(async { Ok(serde_json::json!({})) })
    });
    let mut ids = std::collections::HashSet::new();
    assert!(ids.insert(op1.id));
    assert!(ids.insert(op2.id));
    assert!(ids.insert(op3.id));
    assert!(ids.insert(op4.id));
    assert_eq!(ids.len(), 4);
}

#[test]
fn test_extension_builder() {
    let ext = RuntimeExtension::builder("test")
        .ops(vec![OpDecl::new_sync("test.ping", |_s: &mut OpState| {
            Ok(serde_json::json!({"ok":true}))
        })])
        .build();
    assert_eq!(ext.name, "test");
    assert_eq!(ext.ops.len(), 1);
    assert_eq!(ext.deps.len(), 0);
}

#[test]
fn test_extension_set_resolve() {
    let ext_a = RuntimeExtension::builder("a").build();
    let ext_b = RuntimeExtension::builder("b").deps(&["a"]).build();
    let mut set = ExtensionSet::new();
    set.add(ext_a);
    set.add(ext_b);
    let resolved = set.resolve().unwrap();
    assert_eq!(resolved.len(), 2);
    assert_eq!(resolved[0].name, "a");
    assert_eq!(resolved[1].name, "b");
}

#[test]
fn test_extension_cycle_detection() {
    // B3 fix: detect cycles.
    let ext_a = RuntimeExtension::builder("a").deps(&["b"]).build();
    let ext_b = RuntimeExtension::builder("b").deps(&["a"]).build();
    let mut set = ExtensionSet::new();
    set.add(ext_a);
    set.add(ext_b);
    let result = set.resolve();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), RuntimeErrorKind::ExtensionCycle);
}

#[test]
fn test_extension_set_ops() {
    let ext = RuntimeExtension::builder("a")
        .ops(vec![OpDecl::new_sync("a.op", |_s: &mut OpState| {
            Ok(serde_json::json!({}))
        })])
        .build();
    let mut set = ExtensionSet::new();
    set.add(ext);
    assert_eq!(set.get_all_ops().unwrap().len(), 1);
}

#[test]
fn test_extension_set_ops_in_dep_order() {
    // B2 fix: ops from extensions should be retrievable in resolve order.
    let ext_a = RuntimeExtension::builder("a")
        .ops(vec![OpDecl::new_sync("a.op", |_s: &mut OpState| {
            Ok(serde_json::json!({}))
        })])
        .build();
    let ext_b = RuntimeExtension::builder("b")
        .deps(&["a"])
        .ops(vec![OpDecl::new_sync("b.op", |_s: &mut OpState| {
            Ok(serde_json::json!({}))
        })])
        .build();
    let mut set = ExtensionSet::new();
    set.add(ext_b);
    set.add(ext_a);
    let ops = set.get_all_ops().unwrap();
    assert_eq!(ops[0].name, "a.op");
    assert_eq!(ops[1].name, "b.op");
}

#[test]
fn test_extension_set_lifecycle_order() {
    // B2 fix: lifecycle hooks run in dependency order.
    let init_order = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let ext_a = RuntimeExtension::builder("a")
        .init_state({
            let order = init_order.clone();
            move |_state: &mut OpState| {
                order.lock().unwrap().push("a_init");
            }
        })
        .build();
    let ext_b = RuntimeExtension::builder("b")
        .deps(&["a"])
        .init_state({
            let order = init_order.clone();
            move |_state: &mut OpState| {
                order.lock().unwrap().push("b_init");
            }
        })
        .build();
    let mut set = ExtensionSet::new();
    set.add(ext_b);
    set.add(ext_a);
    let mut state = OpState::new();
    set.init_all_states(&mut state).unwrap();
    let order = init_order.lock().unwrap().clone();
    assert_eq!(order, vec!["a_init", "b_init"]);
}

#[test]
fn test_runtime_new() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default()).unwrap();
    rt.init().unwrap();
    assert!(rt.is_initialized());
    assert_eq!(rt.op_count(), 0);
}

#[test]
fn test_runtime_with_extension() {
    let montrs = montrs_runtime::montrs_ext::init();
    let mut rt = MontrsRuntime::new(RuntimeOptions {
        extensions: vec![montrs],
        ..Default::default()
    })
    .unwrap();
    rt.init().unwrap();
    let result = rt.op_sync("montrs.ping", None).unwrap();
    assert_eq!(result["ok"], true);
    assert!(rt.op_count() >= 3);
}

#[test]
fn test_runtime_register_op() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default()).unwrap();
    rt.register_op(OpDecl::new_sync("custom", |_s: &mut OpState| {
        Ok(serde_json::json!({"custom":true}))
    }));
    let result = rt.op_sync("custom", None).unwrap();
    assert_eq!(result["custom"], true);
}

#[test]
fn test_runtime_op_not_found() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default()).unwrap();
    let result = rt.op_sync("nonexistent", None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), RuntimeErrorKind::OpNotFound);
}

#[test]
fn test_runtime_shutdown() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default()).unwrap();
    rt.init().unwrap();
    assert!(rt.is_initialized());
    rt.shutdown().unwrap();
    assert!(!rt.is_initialized());
}

#[test]
fn test_extension_count() {
    // B7 fix: extension_count() returns extension count, not op count.
    let ext = RuntimeExtension::builder("test")
        .ops(vec![OpDecl::new_sync("t.op", |_s: &mut OpState| {
            Ok(serde_json::json!({}))
        })])
        .build();
    let rt = MontrsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    })
    .unwrap();
    assert_eq!(rt.extension_count(), 1);
    assert_eq!(rt.op_count(), 1);
}

#[test]
fn test_runtime_error_codes() {
    let err = RuntimeError::op_not_found("test");
    assert_eq!(err.code(), "op_not_found");
    let err = RuntimeError::resource("broken");
    assert_eq!(err.code(), "resource");
    let err = RuntimeError::out_of_memory();
    assert_eq!(err.code(), "out_of_memory");
    assert!(!err.suggested_fixes().is_empty());
}

#[test]
fn test_runtime_error_suggested_fixes() {
    let err = RuntimeError::op_not_found("x");
    let fixes = err.suggested_fixes();
    assert!(!fixes.is_empty());
    assert!(fixes[0].contains("op"));
}

#[tokio::test]
async fn test_op_result_async_mismatch() {
    // B13 fix: executing a sync op as async returns RuntimeError.
    let mut state = OpState::new();
    let op = OpDecl::new_sync("sync_op", |_s: &mut OpState| {
        Ok(serde_json::json!({}))
    });
    let result = op.execute_async(Arc::new(Mutex::new(state)), None).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), RuntimeErrorKind::OpMismatch);
}

#[tokio::test]
async fn test_runtime_op_async() {
    let montrs = montrs_runtime::montrs_ext::init();
    let mut rt = MontrsRuntime::new(RuntimeOptions {
        extensions: vec![montrs],
        ..Default::default()
    })
    .unwrap();
    rt.init().unwrap();
    let result = rt.op_async("montrs.sleep_ms", None).await.unwrap();
    assert_eq!(result["slept"], true);
}