//! Extension system — pluggable runtime features.
//!
//! Inspired by Deno's `deno_core::extensions`. Each extension provides
//! ops, state initialization, and lifecycle hooks.

use crate::{ops::OpDecl, type_map::OpState};
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

/// ExtensionSet — manages a collection of extensions with dependency resolution.
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
    pub fn resolve(&self) -> Vec<&RuntimeExtension> {
        // Simple topological sort by deps.
        let mut resolved = Vec::new();
        let mut visited = std::collections::HashSet::new();
        for ext in &self.extensions {
            resolve_deps(&self.extensions, ext, &mut visited, &mut resolved);
        }
        resolved
    }

    pub fn get_all_ops(&self) -> Vec<&OpDecl> {
        let mut ops = Vec::new();
        for ext in &self.extensions {
            ops.extend(ext.ops.iter());
        }
        ops
    }

    pub fn init_all_states(&self, state: &mut OpState) {
        for ext in &self.extensions {
            if let Some(ref init) = ext.init_state {
                init(state);
            }
        }
    }

    pub fn start_all(&self, state: &mut OpState) {
        for ext in &self.extensions {
            if let Some(ref start) = ext.on_start {
                start(state);
            }
        }
    }

    pub fn stop_all(&self, state: &mut OpState) {
        for ext in &self.extensions {
            if let Some(ref stop) = ext.on_stop {
                stop(state);
            }
        }
    }
}

impl Default for ExtensionSet {
    fn default() -> Self {
        Self::new()
    }
}

fn resolve_deps<'a>(
    all: &'a [RuntimeExtension],
    ext: &'a RuntimeExtension,
    visited: &mut std::collections::HashSet<&'static str>,
    resolved: &mut Vec<&'a RuntimeExtension>,
) {
    if !visited.insert(ext.name) {
        return;
    }
    for dep_name in ext.deps {
        if let Some(dep) = all.iter().find(|e| e.name == *dep_name) {
            resolve_deps(all, dep, visited, resolved);
        }
    }
    resolved.push(ext);
}
