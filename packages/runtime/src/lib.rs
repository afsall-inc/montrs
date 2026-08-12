//! MontRS runtime — general-purpose Rust runtime with ops, extensions,
//! resource table, event loop, memory optimization, and module loading.
//!
//! Architecture (inspired by Deno's core):
//! - `OpState` — TypeMap for extension state
//! - `RuntimeExtension` — pluggable extension with ops, state, lifecycle
//! - `OpDecl` — typed operation declaration (sync/async, with/without input)
//! - `ResourceTable` — typed handles identified by ResourceId
//! - `EventLoop` — tokio-based async task management
//! - `MontrsRuntime` — the main runtime struct
//! - `ModuleLoader` — trait for loading Rust/WASM modules
//! - `Arena` — bump allocator for fast allocation
//! - `TaggedValue` — 64-bit tagged value representation
//! - `RuntimeError` — structured error type with stable codes

pub mod error;
pub mod event_loop;
pub mod extensions;
pub mod memory;
pub mod modules;
pub mod ops;
pub mod resource_table;
pub mod runtime;
pub mod type_map;

pub use error::{RuntimeError, RuntimeErrorKind};
pub use event_loop::{EventLoop, EventLoopMsg, TaskId};
pub use extensions::{ExtensionSet, RuntimeExtension, RuntimeExtensionBuilder};
pub use memory::{Arena, BitField, TaggedValue};
pub use modules::{
    FileModuleLoader, Module, ModuleKind, ModuleLoader, ModuleSource,
};
pub use ops::{AsyncOpResult, OpDecl, OpFn, OpId, OpResult};
pub use resource_table::{
    FileResource, Resource, ResourceId, ResourceTable, TcpStreamResource,
};
pub use runtime::{MontrsRuntime, RuntimeOptions};
pub use type_map::{OpState, TypeMap};

/// A convenience re-export for building a MontRS runtime with common extensions.
pub mod prelude {
    pub use crate::{
        error::*, EventLoop, MontrsRuntime, OpDecl, OpId, OpResult, OpState,
        Resource, ResourceId, ResourceTable, RuntimeError, RuntimeExtension,
        RuntimeExtensionBuilder, RuntimeOptions, TypeMap,
    };
}

/// The MontRS-specific runtime extension — provides ops optimized for
/// MontRS applications: fast routing, plate caching, ORM connection
/// pooling, env config, and agent integration.
pub mod montrs_ext {
    use crate::ops::OpDecl;
    use crate::resource_table::ResourceTable;
    use crate::type_map::OpState;
    use crate::RuntimeExtension;

    /// Initialize the MontRS extension.
    pub fn init() -> RuntimeExtension {
        RuntimeExtension::builder("montrs")
            .ops(vec![
                OpDecl::new_sync("montrs.ping", |_state: &mut OpState| {
                    Ok(serde_json::json!({ "ok": true }))
                }),
                OpDecl::new_sync(
                    "montrs.resource_count",
                    |state: &mut OpState| {
                        let count = state
                            .get::<ResourceTable>()
                            .map(|rt| rt.len())
                            .unwrap_or(0);
                        Ok(serde_json::json!({
                            "resource_count": count,
                        }))
                    },
                ),
                OpDecl::new_async("montrs.sleep_ms", |state: &mut OpState| {
                    let _ = state;
                    Box::pin(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            100,
                        ))
                        .await;
                        Ok(serde_json::json!({ "slept": true }))
                    })
                }),
            ])
            .init_state(|state: &mut OpState| {
                state.put(MontrsState::default());
            })
            .build()
    }

    /// State for the MontRS extension.
    #[derive(Default)]
    pub struct MontrsState {
        pub init_count: u64,
        pub config: MontrsConfig,
    }

    impl MontrsState {
        /// Increment the init counter (called by the extension itself).
        pub fn increment(&mut self) {
            self.init_count += 1;
        }
    }

    /// Configuration for MontRS runtime.
    #[derive(Default)]
    pub struct MontrsConfig {
        pub enable_agent: bool,
        pub enable_orm_pooling: bool,
        pub max_orm_connections: u32,
        pub enable_plate_cache: bool,
        pub enable_ssr_signals: bool,
    }
}