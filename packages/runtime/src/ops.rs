//! Operation system — typed async/sync operations for the runtime.
//!
//! Inspired by Deno's `deno_core::ops`. Ops are functions that can be
//! called from within the runtime with access to OpState.

use crate::type_map::OpState;
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
};

/// A unique operation identifier.
pub type OpId = u16;

/// The result of a synchronous operation.
pub type OpResult<T = serde_json::Value> = Result<T, OpError>;

/// The result of an asynchronous operation.
pub type AsyncOpResult<T = serde_json::Value> =
    Pin<Box<dyn Future<Output = OpResult<T>> + Send>>;

/// An operation declaration — the core abstraction for runtime extensions.
#[derive(Clone)]
pub struct OpDecl {
    /// Unique operation ID (auto-assigned if 0).
    pub id: OpId,
    /// Human-readable name.
    pub name: &'static str,
    /// Whether this op is async.
    pub is_async: bool,
    /// The operation function.
    pub op_fn: OpFn,
}

/// A sync operation function.
pub type SyncOp = Arc<dyn Fn(&mut OpState) -> OpResult + Send + Sync>;
/// An async operation function.
pub type AsyncOp = Arc<dyn Fn(&mut OpState) -> AsyncOpResult + Send + Sync>;
/// A sync operation with JSON input.
pub type SyncOpWithInput =
    Arc<dyn Fn(&mut OpState, serde_json::Value) -> OpResult + Send + Sync>;
/// An async operation with JSON input.
pub type AsyncOpWithInput =
    Arc<dyn Fn(&mut OpState, serde_json::Value) -> AsyncOpResult + Send + Sync>;

/// The function signature for an operation.
#[derive(Clone)]
pub enum OpFn {
    Sync(SyncOp),
    Async(AsyncOp),
    SyncWithInput(SyncOpWithInput),
    AsyncWithInput(AsyncOpWithInput),
}

impl std::fmt::Debug for OpFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(_) => write!(f, "Sync"),
            Self::Async(_) => write!(f, "Async"),
            Self::SyncWithInput(_) => write!(f, "SyncWithInput"),
            Self::AsyncWithInput(_) => write!(f, "AsyncWithInput"),
        }
    }
}

impl OpDecl {
    pub fn new_sync(
        name: &'static str,
        f: impl Fn(&mut OpState) -> OpResult + Send + Sync + 'static,
    ) -> Self {
        static NEXT_ID: AtomicU16 = AtomicU16::new(1);
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            is_async: false,
            op_fn: OpFn::Sync(Arc::new(f)),
        }
    }

    pub fn new_async(
        name: &'static str,
        f: impl Fn(&mut OpState) -> AsyncOpResult + Send + Sync + 'static,
    ) -> Self {
        static NEXT_ID: AtomicU16 = AtomicU16::new(1);
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            is_async: true,
            op_fn: OpFn::Async(Arc::new(f)),
        }
    }

    pub fn new_sync_with_input(
        name: &'static str,
        f: impl Fn(&mut OpState, serde_json::Value) -> OpResult
        + Send
        + Sync
        + 'static,
    ) -> Self {
        static NEXT_ID: AtomicU16 = AtomicU16::new(1);
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            is_async: false,
            op_fn: OpFn::SyncWithInput(Arc::new(f)),
        }
    }

    pub fn new_async_with_input(
        name: &'static str,
        f: impl Fn(&mut OpState, serde_json::Value) -> AsyncOpResult
        + Send
        + Sync
        + 'static,
    ) -> Self {
        static NEXT_ID: AtomicU16 = AtomicU16::new(1);
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            is_async: true,
            op_fn: OpFn::AsyncWithInput(Arc::new(f)),
        }
    }

    /// Execute the operation with the given state and optional input.
    pub fn execute(
        &self,
        state: &mut OpState,
        input: Option<serde_json::Value>,
    ) -> OpResult {
        match &self.op_fn {
            OpFn::Sync(f) => f(state),
            OpFn::Async(_) => {
                Err(OpError("async op requires await".to_string()))
            }
            OpFn::SyncWithInput(f) => {
                f(state, input.unwrap_or(serde_json::Value::Null))
            }
            OpFn::AsyncWithInput(_) => {
                Err(OpError("async op requires await".to_string()))
            }
        }
    }

    /// Execute the async operation.
    pub fn execute_async(
        &self,
        state: &mut OpState,
        input: Option<serde_json::Value>,
    ) -> AsyncOpResult {
        match &self.op_fn {
            OpFn::Async(f) => f(state),
            OpFn::AsyncWithInput(f) => {
                f(state, input.unwrap_or(serde_json::Value::Null))
            }
            OpFn::Sync(_) | OpFn::SyncWithInput(_) => Box::pin(async move {
                Err(OpError("sync op cannot be awaited".to_string()))
            }),
        }
    }
}

/// Operation error.
#[derive(Debug, Clone)]
pub struct OpError(pub String);

impl std::fmt::Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpError: {}", self.0)
    }
}

impl std::error::Error for OpError {}

impl From<String> for OpError {
    fn from(s: String) -> Self {
        OpError(s)
    }
}
impl From<serde_json::Error> for OpError {
    fn from(e: serde_json::Error) -> Self {
        OpError(e.to_string())
    }
}
