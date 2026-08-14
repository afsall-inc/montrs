//! TypeMap — a type-indexed map used as `OpState` for storing runtime extension state.
//!
//! Stores one value of each type, identified by `TypeId`.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// A type-indexed map that stores one value of each type. Used as `OpState`
/// for the runtime, where extensions store and retrieve their state.
#[derive(Default)]
pub struct TypeMap {
    entries: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Insert a value of type `T` into the state.
    pub fn put<T: Send + 'static>(&mut self, t: T) {
        self.entries.insert(TypeId::of::<T>(), Box::new(t));
    }

    /// Get a reference to a value of type `T`.
    pub fn get<T: Send + 'static>(&self) -> Option<&T> {
        self.entries
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
    }

    /// Get a mutable reference to a value of type `T`.
    pub fn get_mut<T: Send + 'static>(&mut self) -> Option<&mut T> {
        self.entries
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut::<T>())
    }

    /// Remove a value of type `T` from the state.
    pub fn take<T: Send + 'static>(&mut self) -> Option<T> {
        self.entries
            .remove(&TypeId::of::<T>())
            .and_then(|b| b.downcast::<T>().ok().map(|b| *b))
    }

    /// Check if a value of type `T` exists.
    pub fn contains<T: Send + 'static>(&self) -> bool {
        self.entries.contains_key(&TypeId::of::<T>())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// The operation state — a type map for extension state, accessible from ops.
pub type OpState = TypeMap;
