//! Deterministic state management for MontRS.
//!
//! Inspired by Zustand's simplicity and XState's state machine capabilities.
//! Provides stores, selectors, middleware, and typed state machines.
//!
//! The core is platform-independent. Leptos integration is optional.

/// Errors for store and machine operations.
pub mod error;

/// Zustand-style stores with selectors and middleware.
pub mod store;

/// Undo/redo time travel for stores.
pub mod time_travel;

/// XState-style typed state machines.
pub mod machine;

/// Declarative store and selector macros.
pub mod macros;

/// Leptos hooks (feature-gated).
#[cfg(feature = "leptos")]
pub mod leptos;

pub use error::{StateError, StateResult};
pub use machine::{
    Action, AssignAction, DataEvent, FunctionAction, FunctionGuard, Guard,
    HistoryEntry, HistoryMachine, HistoryTracker, LogAction, Machine,
    MachineBuilder, MachineError, MachineHistory, MachineState, StateMachine,
    StateNode, StringEvent, Transition,
};
pub use store::{
    FieldSelector, LoggerMiddleware, Middleware, MiddlewareChain, SimpleStore,
    Store, StoreContext, StoreSlice, ValidationMiddleware,
};
pub use time_travel::{Snapshot, TimeTravel};

/// Type aliases matching the reference API.
pub type StoreId = String;
pub type MachineId = String;
pub type StateId = String;
pub type EventId = String;

/// Create a new simple store with an initial value.
pub fn create_store<T>(initial: T) -> StoreContext<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    StoreContext::new(SimpleStore::new(initial))
}
