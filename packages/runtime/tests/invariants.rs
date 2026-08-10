//! Invariant tests for montrs-runtime.

use montrs_runtime::{prelude::*, *};

#[test]
fn test_arena_alloc() {
    let arena = Arena::new(1024);
    let (_ptr, size) = arena.alloc(100).expect("alloc");
    assert_eq!(size, 104); // aligned to 8
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
    // Allocate more than the arena size.
    assert!(arena.alloc(100).is_none());
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
    table.close(id);
    assert_eq!(table.len(), 0);
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
fn test_op_error_display() {
    let err = OpError("boom".to_string());
    assert!(err.to_string().contains("boom"));
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
    let resolved = set.resolve();
    assert_eq!(resolved.len(), 2);
    // "a" must come before "b".
    assert_eq!(resolved[0].name, "a");
    assert_eq!(resolved[1].name, "b");
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
    assert_eq!(set.get_all_ops().len(), 1);
}

#[test]
fn test_runtime_new() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default());
    rt.init();
    assert!(rt.is_initialized());
    assert_eq!(rt.op_count(), 0);
}

#[test]
fn test_runtime_with_extension() {
    let montrs = montrs_runtime::montrs_ext::init();
    let mut rt = MontrsRuntime::new(RuntimeOptions {
        extensions: vec![montrs],
        ..Default::default()
    });
    rt.init();
    let result = rt.op_sync("montrs.ping", None).unwrap();
    assert_eq!(result["ok"], true);
    assert!(rt.op_count() >= 3);
}

#[test]
fn test_runtime_register_op() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default());
    rt.register_op(OpDecl::new_sync("custom", |_s: &mut OpState| {
        Ok(serde_json::json!({"custom":true}))
    }));
    let result = rt.op_sync("custom", None).unwrap();
    assert_eq!(result["custom"], true);
}

#[test]
fn test_runtime_op_not_found() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default());
    let result = rt.op_sync("nonexistent", None);
    assert!(result.is_err());
}

#[test]
fn test_runtime_shutdown() {
    let mut rt = MontrsRuntime::new(RuntimeOptions::default());
    rt.init();
    assert!(rt.is_initialized());
    rt.shutdown();
    assert!(!rt.is_initialized());
}
