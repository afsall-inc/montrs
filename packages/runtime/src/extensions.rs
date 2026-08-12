//! Extension system — pluggable runtime features.
//!
//! Inspired by Deno's `deno_core::extensions`. Each extension provides
//! ops, state initialization, and lifecycle hooks.

use crate::error::RuntimeError;
use crate::ops::OpDecl;
use crate::type_map::OpState;
use std::collections::HashSet;
use std::sync::Arc;

/// A state initialization/teardown callback.
pub type StateFn = Arc<dyn Fn(&mut OpState) + Send + Sync>;

/// A runtime extension — groups ops, state, and initialization.
pub struct RuntimeExtension {
    /// Human-readable name of the extension.
    pub name: &'static str,
    /// Other extensions this one depends on.
    pub deps: &'static [&'static str],
    /// Ops provided by this extension.
    pub ops: Vec<OpDecl>,
    /// State initialization function.
    pub init_state: Option<StateFn>,
    /// Called when the runtime starts.
    pub on_start: Option<StateFn>,
    /// Called when the runtime stops.
    pub on_stop: Option<StateFn>,
}

impl std::fmt::Debug for RuntimeExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeExtension")
            .field("name", &self.name)
            .field("deps", &self.deps)
            .field("ops", &self.ops.len())
            .finish()
    }
}

impl RuntimeExtension {
    /// Create a new extension builder.
    pub fn builder(name: &'static str) -> RuntimeExtensionBuilder {
        RuntimeExtensionBuilder {
            name,
            deps: &[],
            ops: Vec::new(),
            init_state: None,
            on_start: None,
            on_stop: None,
        }
    }
}

/// Builder for RuntimeExtension.
pub struct RuntimeExtensionBuilder {
    name: &'static str,
    deps: &'static [&'static str],
    ops: Vec<OpDecl>,
    init_state: Option<StateFn>,
    on_start: Option<StateFn>,
    on_stop: Option<StateFn>,
}

impl RuntimeExtensionBuilder {
    pub fn deps(mut self, deps: &'static [&'static str]) -> Self {
        self.deps = deps;
        self
    }
    pub fn ops(mut self, ops: Vec<OpDecl>) -> Self {
        self.ops = ops;
        self
    }
    pub fn init_state(
        mut self,
        f: impl Fn(&mut OpState) + Send + Sync + 'static,
    ) -> Self {
        self.init_state = Some(Arc::new(f));
        self
    }
    pub fn on_start(
        mut self,
        f: impl Fn(&mut OpState) + Send + Sync + 'static,
    ) -> Self {
        self.on_start = Some(Arc::new(f));
        self
    }
    pub fn on_stop(
        mut self,
        f: impl Fn(&mut OpState) + Send + Sync + 'static,
    ) -> Self {
        self.on_stop = Some(Arc::new(f));
        self
    }
    pub fn build(self) -> RuntimeExtension {
        RuntimeExtension {
            name: self.name,
            deps: self.deps,
            ops: self.ops,
            init_state: self.init_state,
            on_start: self.on_start,
            on_stop: self.on_stop,
        }
    }
}

/// ExtensionSet — manages a collection of extensions with dependency resolution
/// and cycle detection. Lifecycle hooks run in resolved (dependency-first) order.
pub struct ExtensionSet {
    extensions: Vec<RuntimeExtension>,
}

impl ExtensionSet {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    pub fn add(&mut self, ext: RuntimeExtension) {
        self.extensions.push(ext);
    }

    pub fn add_all(&mut self, exts: Vec<RuntimeExtension>) {
        self.extensions.extend(exts);
    }

    /// Resolve all extensions, respecting dependencies. Returns in dependency order.
    /// Errors on cycles with a detailed RuntimeError.
    pub fn resolve(&self) -> Result<Vec<&RuntimeExtension>, RuntimeError> {
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();
        // Track the current DFS path for cycle detection.
        let mut in_stack = HashSet::new();

        for ext in &self.extensions {
            self.resolve_deps(ext, &mut visited, &mut in_stack, &mut resolved)?;
        }
        Ok(resolved)
    }

    /// DFS-based topological sort with cycle detection (B3 fix).
    fn resolve_deps<'a>(
        &'a self,
        ext: &'a RuntimeExtension,
        visited: &mut HashSet<&'static str>,
        in_stack: &mut HashSet<&'static str>,
        resolved: &mut Vec<&'a RuntimeExtension>,
    ) -> Result<(), RuntimeError> {
        // Cycle detection: if we're already in the current recursion stack, we have a cycle.
        if in_stack.contains(ext.name) {
            let cycle_path: Vec<&str> = resolved
                .iter()
                .rev()
                .take_while(|e| e.name != ext.name)
                .map(|e| e.name)
                .chain(std::iter::once(ext.name))
                .collect();
            return Err(RuntimeError::extension_cycle(&cycle_path));
        }

        if !visited.insert(ext.name) {
            return Ok(());
        }

        in_stack.insert(ext.name);

        for dep_name in ext.deps {
            let dep = self
                .extensions
                .iter()
                .find(|e| e.name == *dep_name)
                .ok_or_else(|| RuntimeError::missing_dependency(ext.name, dep_name))?;
            self.resolve_deps(dep, visited, in_stack, resolved)?;
        }

        in_stack.remove(ext.name);
        resolved.push(ext);
        Ok(())
    }

    /// Returns all ops in dependency order (B2 fix).
    pub fn get_all_ops(&self) -> Result<Vec<&OpDecl>, RuntimeError> {
        let resolved = self.resolve()?;
        let mut ops = Vec::new();
        for ext in resolved {
            ops.extend(ext.ops.iter());
        }
        Ok(ops)
    }

    /// Initialize all extension states in dependency order (B2 fix).
    pub fn init_all_states(&self, state: &mut OpState) -> Result<(), RuntimeError> {
        for ext in self.resolve()? {
            if let Some(ref init) = ext.init_state {
                init(state);
            }
        }
        Ok(())
    }

    /// Start all extensions in dependency order (B2 fix).
    pub fn start_all(&self, state: &mut OpState) -> Result<(), RuntimeError> {
        for ext in self.resolve()? {
            if let Some(ref start) = ext.on_start {
                start(state);
            }
        }
        Ok(())
    }

    /// Stop all extensions in reverse dependency order (so dependents stop first).
    pub fn stop_all(&self, state: &mut OpState) -> Result<(), RuntimeError> {
        let mut resolved = self.resolve()?;
        resolved.reverse();
        for ext in resolved {
            if let Some(ref stop) = ext.on_stop {
                stop(state);
            }
        }
        Ok(())
    }

    /// Count of registered extensions.
    pub fn extension_count(&self) -> usize {
        self.extensions.len()
    }
}

impl Default for ExtensionSet {
    fn default() -> Self {
        Self::new()
    }
}