//! Worker — an isolated runtime instance with its own OpState, extensions,
//! resource table, and event loop. Inspired by Deno's `MainWorker`.

use crate::error::RuntimeError;
use crate::event_loop::EventLoop;
use crate::extensions::{ExtensionSet, RuntimeExtension};
use crate::memory::Arena;
use crate::modules::ModuleLoader;
use crate::ops::OpDecl;
use crate::permissions::Permissions;
use crate::resource_table::ResourceTable;
use crate::type_map::OpState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Options for bootstrapping a worker.
pub struct WorkerBootstrapOptions {
    pub extensions: Vec<RuntimeExtension>,
    pub module_loader: Option<Box<dyn ModuleLoader>>,
    pub max_tasks: usize,
    pub arena_size: usize,
    pub permissions: Permissions,
}

impl Clone for WorkerBootstrapOptions {
    fn clone(&self) -> Self {
        Self {
            extensions: self.extensions.clone(),
            module_loader: None,
            max_tasks: self.max_tasks,
            arena_size: self.arena_size,
            permissions: self.permissions.clone(),
        }
    }
}

impl Default for WorkerBootstrapOptions {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            module_loader: None,
            max_tasks: 256,
            arena_size: 256 * 1024,
            permissions: Permissions::none(),
        }
    }
}

/// A handle to a running worker task.
pub struct WorkerHandle {
    pub id: u64,
    pub join_handle: tokio::task::JoinHandle<anyhow::Result<()>>,
}

/// An isolated runtime worker — owns its own OpState, event loop, and resource table.
pub struct Worker {
    pub id: u64,
    pub state: OpState,
    pub event_loop: EventLoop,
    extensions: ExtensionSet,
    ops_by_id: HashMap<u16, OpDecl>,
    ops_by_name: HashMap<&'static str, OpDecl>,
    arena: Arena,
    pub permissions: Permissions,
    initialized: bool,
}

impl Worker {
    /// Bootstrap a new worker.
    pub fn bootstrap(id: u64, options: WorkerBootstrapOptions) -> Result<Self, RuntimeError> {
        let mut extensions = ExtensionSet::new();
        extensions.add_all(options.extensions);
        let mut state = OpState::new();
        state.put(ResourceTable::new());
        state.put(EventLoop::new());
        state.put(options.permissions.clone());
        extensions.init_all_states(&mut state)?;
        let mut ops_by_id = HashMap::new();
        let mut ops_by_name = HashMap::new();
        for op in extensions.get_all_ops()? {
            ops_by_id.insert(op.id, op.clone());
            ops_by_name.insert(op.name, op.clone());
        }
        let arena = Arena::new(options.arena_size);
        Ok(Self {
            id,
            state,
            event_loop: EventLoop::new(),
            extensions,
            ops_by_id,
            ops_by_name,
            arena,
            permissions: options.permissions,
            initialized: false,
        })
    }

    pub fn init(&mut self) -> Result<(), RuntimeError> {
        self.extensions.start_all(&mut self.state)?;
        self.event_loop.start();
        self.initialized = true;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool { self.initialized }
    pub fn op_count(&self) -> usize { self.ops_by_id.len() }
    pub fn arena(&self) -> &Arena { &self.arena }
    pub fn arena_mut(&mut self) -> &mut Arena { &mut self.arena }
    pub fn id(&self) -> u64 { self.id }

    /// Execute a synchronous operation.
    pub fn op_sync(&mut self, name: &str, input: Option<serde_json::Value>) -> Result<serde_json::Value, RuntimeError> {
        let op = self.ops_by_name.get(name).cloned().ok_or_else(|| RuntimeError::op_not_found(name))?;
        op.execute(&mut self.state, input)
    }

    /// Execute an async operation.
    pub async fn op_async(&mut self, name: &str, input: Option<serde_json::Value>) -> Result<serde_json::Value, RuntimeError> {
        let op = self.ops_by_name.get(name).cloned().ok_or_else(|| RuntimeError::op_not_found(name))?;
        let shared = Arc::new(Mutex::new(std::mem::take(&mut self.state)));
        let result = op.execute_async(shared.clone(), input).await;
        self.state = Arc::try_unwrap(shared).ok().unwrap().into_inner();
        result
    }

    /// Register a new op.
    pub fn register_op(&mut self, op: OpDecl) {
        self.ops_by_id.insert(op.id, op.clone());
        self.ops_by_name.insert(op.name, op.clone());
    }

    /// Run the event loop.
    pub async fn run(&mut self) {
        self.event_loop.run().await;
    }

    /// Shutdown the worker.
    pub fn shutdown(&mut self) -> Result<(), RuntimeError> {
        self.event_loop.stop();
        self.extensions.stop_all(&mut self.state)?;
        self.initialized = false;
        Ok(())
    }

    /// Spawn this worker as a background task, returning a WorkerHandle.
    pub fn spawn(mut self) -> WorkerHandle {
        let id = self.id;
        let join_handle = tokio::spawn(async move {
            self.event_loop.run().await;
            Ok(())
        });
        WorkerHandle { id, join_handle }
    }
}

/// A pool of workers.
pub struct WorkerPool {
    workers: HashMap<u64, WorkerHandle>,
    next_id: u64,
    bootstrap_options: WorkerBootstrapOptions,
}

impl WorkerPool {
    pub fn new(options: WorkerBootstrapOptions) -> Self {
        Self { workers: HashMap::new(), next_id: 1, bootstrap_options: options }
    }

    pub fn spawn(&mut self) -> Result<u64, RuntimeError> {
        let id = self.next_id;
        self.next_id += 1;
        let mut worker = Worker::bootstrap(id, self.bootstrap_options.clone())?;
        worker.init()?;
        let handle = worker.spawn();
        self.workers.insert(id, handle);
        Ok(id)
    }

    pub fn get(&self, id: u64) -> Option<&WorkerHandle> {
        self.workers.get(&id)
    }

    pub fn abort_all(&mut self) {
        self.workers.clear();
    }

    pub fn active_count(&self) -> usize {
        self.workers.len()
    }
}

impl Default for WorkerPool {
    fn default() -> Self {
        Self::new(WorkerBootstrapOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::OpDecl;
    use crate::type_map::OpState;

    #[test]
    fn test_worker_bootstrap() {
        let mut worker = Worker::bootstrap(1, WorkerBootstrapOptions {
            permissions: Permissions::all(),
            ..Default::default()
        }).unwrap();
        worker.init().unwrap();
        assert!(worker.is_initialized());
        assert_eq!(worker.op_count(), 0);
    }

    #[test]
    fn test_worker_op() {
        let mut worker = Worker::bootstrap(1, WorkerBootstrapOptions {
            permissions: Permissions::all(),
            ..Default::default()
        }).unwrap();
        worker.register_op(OpDecl::new_sync("ping", |_s: &mut OpState| {
            Ok(serde_json::json!({"pong": true}))
        }));
        let result = worker.op_sync("ping", None).unwrap();
        assert_eq!(result["pong"], true);
    }

    #[test]
    fn test_worker_permissions() {
        let worker = Worker::bootstrap(1, WorkerBootstrapOptions {
            permissions: Permissions::none(),
            ..Default::default()
        }).unwrap();
        assert!(worker.permissions.check_fs_read("/etc").is_err());
    }

    #[tokio::test]
    async fn test_worker_pool() {
        let mut pool = WorkerPool::new(WorkerBootstrapOptions {
            permissions: Permissions::all(),
            ..Default::default()
        });
        let id = pool.spawn().unwrap();
        assert!(pool.get(id).is_some());
        assert_eq!(pool.active_count(), 1);
    }
}