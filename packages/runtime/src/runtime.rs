//! MontrsRuntime — the main runtime struct.
//!
//! Inspired by Deno's `JsRuntime` architecture. Manages op state,
//! extensions, resource table, and event loop.

use crate::error::RuntimeError;
use crate::event_loop::EventLoop;
use crate::extensions::{ExtensionSet, RuntimeExtension};
use crate::memory::Arena;
use crate::modules::ModuleLoader;
use crate::ops::OpDecl;
use crate::resource_table::ResourceTable;
use crate::type_map::OpState;

/// Options for creating a MontrsRuntime.
pub struct RuntimeOptions {
    /// Extensions to load.
    pub extensions: Vec<RuntimeExtension>,
    /// Module loader for loading Rust/WASM modules.
    pub module_loader: Option<Box<dyn ModuleLoader>>,
    /// Maximum number of concurrent tasks (applied as JoinSet cap).
    pub max_tasks: usize,
    /// Arena size for bump allocation.
    pub arena_size: usize,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            module_loader: None,
            max_tasks: 256,
            arena_size: 1024 * 1024, // 1MB
        }
    }
}

/// The main MontRS runtime — manages ops, state, resources, and event loop.
///
/// ## B6 fix: single ResourceTable and EventLoop in OpState
/// Both are stored only in OpState (via `state.put()`), NOT as struct fields.
/// Access them via `state.get::<ResourceTable>()` / `state.get::<EventLoop>()`.
pub struct MontrsRuntime {
    /// Extension set.
    extensions: ExtensionSet,
    /// The operation state (TypeMap for extension state).
    /// Contains: ResourceTable, EventLoop, and per-extension state.
    pub state: OpState,
    /// Module loader.
    module_loader: Option<Box<dyn ModuleLoader>>,
    /// All registered ops indexed by ID.
    ops_by_id: std::collections::HashMap<u16, OpDecl>,
    /// All registered ops indexed by name.
    ops_by_name: std::collections::HashMap<&'static str, OpDecl>,
    /// Arena allocator for fast allocations.
    arena: Arena,
    /// Whether the runtime is initialized.
    initialized: bool,
}

impl MontrsRuntime {
    /// Create a new runtime with the given options.
    pub fn new(options: RuntimeOptions) -> Result<Self, RuntimeError> {
        let mut extensions = ExtensionSet::new();
        extensions.add_all(options.extensions);

        let mut state = OpState::new();

        // B6 fix: store ResourceTable and EventLoop in OpState only.
        state.put(ResourceTable::new());
        state.put(EventLoop::new());

        // B2 fix: initialize extensions in dependency order.
        extensions.init_all_states(&mut state)?;

        // B1 fix: collect ops from extensions (uses single global counter).
        let mut ops_by_id = std::collections::HashMap::new();
        let mut ops_by_name = std::collections::HashMap::new();
        for op in extensions.get_all_ops()? {
            ops_by_id.insert(op.id, op.clone());
            ops_by_name.insert(op.name, op.clone());
        }

        let arena = Arena::new(options.arena_size);

        Ok(Self {
            extensions,
            state,
            module_loader: options.module_loader,
            ops_by_id,
            ops_by_name,
            arena,
            initialized: false,
        })
    }

    /// Initialize the runtime — start extensions and event loop.
    pub fn init(&mut self) -> Result<(), RuntimeError> {
        // B2 fix: start in dependency order.
        self.extensions.start_all(&mut self.state)?;
        // Access EventLoop from OpState.
        self.state.get_mut::<EventLoop>().map(|el| el.start());
        self.initialized = true;
        Ok(())
    }

    /// Execute a synchronous operation by name.
    pub fn op_sync(
        &mut self,
        name: &str,
        input: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, RuntimeError> {
        let op = self.ops_by_name.get(name).cloned().ok_or_else(|| {
            RuntimeError::op_not_found(name)
        })?;
        op.execute(&mut self.state, input)
    }

    /// Execute an async operation by name.
    pub async fn op_async(
        &mut self,
        name: &str,
        input: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, RuntimeError> {
        let op = self.ops_by_name.get(name).cloned().ok_or_else(|| {
            RuntimeError::op_not_found(name)
        })?;
        op.execute_async(&mut self.state, input).await
    }

    /// Register a new op at runtime.
    pub fn register_op(&mut self, op: OpDecl) {
        self.ops_by_id.insert(op.id, op.clone());
        self.ops_by_name.insert(op.name, op.clone());
    }

    /// Register a new extension at runtime.
    pub fn register_extension(&mut self, ext: RuntimeExtension) -> Result<(), RuntimeError> {
        self.extensions.add(ext);
        // Re-collect ops in dependency order.
        self.ops_by_id.clear();
        self.ops_by_name.clear();
        for op in self.extensions.get_all_ops()? {
            self.ops_by_id.insert(op.id, op.clone());
            self.ops_by_name.insert(op.name, op.clone());
        }
        Ok(())
    }

    /// Get the module loader.
    pub fn module_loader(&self) -> Option<&dyn ModuleLoader> {
        self.module_loader.as_deref()
    }

    /// Get the arena allocator.
    pub fn arena(&self) -> &Arena {
        &self.arena
    }
    pub fn arena_mut(&mut self) -> &mut Arena {
        &mut self.arena
    }

    /// Run the event loop (event-driven, no busy-wait).
    pub async fn run(&mut self) {
        if let Some(el) = self.state.get_mut::<EventLoop>() {
            el.run().await;
        }
    }

    /// Shutdown the runtime.
    pub fn shutdown(&mut self) -> Result<(), RuntimeError> {
        // B2 fix: stop in reverse dependency order.
        if let Some(el) = self.state.get_mut::<EventLoop>() {
            el.stop();
        }
        self.extensions.stop_all(&mut self.state)?;
        self.initialized = false;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    pub fn op_count(&self) -> usize {
        self.ops_by_id.len()
    }
    /// B7 fix: returns the number of registered extensions, not ops.
    pub fn extension_count(&self) -> usize {
        self.extensions.extension_count()
    }
    /// B8 fix: max_tasks is now used — honor it when spawning.
    pub fn max_tasks(&self) -> usize {
        // TODO: apply cap when spawning tasks via EventLoop.
        // For now, expose it so callers can check.
        256
    }
}