//! MontrsRuntime — the main runtime struct.
//!
//! Inspired by Deno's `JsRuntime` architecture. Manages op state,
//! extensions, resource table, and event loop.

use crate::{
    event_loop::EventLoop,
    extensions::{ExtensionSet, RuntimeExtension},
    memory::Arena,
    modules::ModuleLoader,
    ops::OpDecl,
    resource_table::ResourceTable,
    type_map::OpState,
};

/// Options for creating a MontrsRuntime.
pub struct RuntimeOptions {
    /// Extensions to load.
    pub extensions: Vec<RuntimeExtension>,
    /// Module loader for loading Rust/WASM modules.
    pub module_loader: Option<Box<dyn ModuleLoader>>,
    /// Maximum number of concurrent tasks.
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
pub struct MontrsRuntime {
    /// Extension set.
    extensions: ExtensionSet,
    /// The operation state (TypeMap for extension state).
    pub state: OpState,
    /// Resource table (typed handles).
    pub resources: ResourceTable,
    /// Event loop for async tasks.
    pub event_loop: EventLoop,
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
    pub fn new(options: RuntimeOptions) -> Self {
        let mut extensions = ExtensionSet::new();
        extensions.add_all(options.extensions);

        let mut state = OpState::new();
        state.put(ResourceTable::new());
        state.put(EventLoop::new());

        // Initialize extensions' states.
        extensions.init_all_states(&mut state);

        // Collect all ops.
        let mut ops_by_id = std::collections::HashMap::new();
        let mut ops_by_name = std::collections::HashMap::new();
        for op in extensions.get_all_ops() {
            ops_by_id.insert(op.id, op.clone());
            ops_by_name.insert(op.name, op.clone());
        }

        let arena = Arena::new(options.arena_size);

        Self {
            extensions,
            state,
            resources: ResourceTable::new(),
            event_loop: EventLoop::new(),
            module_loader: options.module_loader,
            ops_by_id,
            ops_by_name,
            arena,
            initialized: false,
        }
    }

    /// Initialize the runtime — start extensions and event loop.
    pub fn init(&mut self) {
        self.extensions.start_all(&mut self.state);
        self.event_loop.start();
        self.initialized = true;
    }

    /// Execute a synchronous operation by name.
    pub fn op_sync(
        &mut self,
        name: &str,
        input: Option<serde_json::Value>,
    ) -> crate::ops::OpResult {
        let op = self.ops_by_name.get(name).cloned().ok_or_else(|| {
            crate::ops::OpError(format!("op not found: {name}"))
        })?;
        op.execute(&mut self.state, input)
    }

    /// Execute an async operation by name.
    pub async fn op_async(
        &mut self,
        name: &str,
        input: Option<serde_json::Value>,
    ) -> crate::ops::OpResult {
        let op = self.ops_by_name.get(name).cloned().ok_or_else(|| {
            crate::ops::OpError(format!("op not found: {name}"))
        })?;
        op.execute_async(&mut self.state, input).await
    }

    /// Register a new op at runtime.
    pub fn register_op(&mut self, op: OpDecl) {
        self.ops_by_id.insert(op.id, op.clone());
        self.ops_by_name.insert(op.name, op.clone());
    }

    /// Register a new extension at runtime.
    pub fn register_extension(&mut self, ext: RuntimeExtension) {
        self.extensions.add(ext);
        // Re-collect ops.
        self.ops_by_id.clear();
        self.ops_by_name.clear();
        for op in self.extensions.get_all_ops() {
            self.ops_by_id.insert(op.id, op.clone());
            self.ops_by_name.insert(op.name, op.clone());
        }
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

    /// Run the event loop.
    pub async fn run(&mut self) {
        self.event_loop.run().await;
    }

    /// Shutdown the runtime.
    pub fn shutdown(&mut self) {
        self.event_loop.stop();
        self.extensions.stop_all(&mut self.state);
        self.initialized = false;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    pub fn op_count(&self) -> usize {
        self.ops_by_id.len()
    }
    pub fn extension_count(&self) -> usize {
        self.extensions.get_all_ops().len()
    }
}
