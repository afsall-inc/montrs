// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Operation system — typed async/sync operations for the runtime.
//!
//! Inspired by Deno's `deno_core::ops`. Ops are functions that can be
//! called from within the runtime with access to OpState.
//!
//! ## State passing
//! - Sync ops receive `&mut OpState` (borrow ends before returning).
//! - Async ops receive `Arc<Mutex<OpState>>` so they can mutate state
//!   across `.await` points (the boxed future must be `'static`).

use crate::{error::RuntimeError, type_map::OpState};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
};
use tokio::sync::Mutex;

/// A unique operation identifier.
pub type OpId = u16;

/// The result of a synchronous operation.
pub type OpResult = Result<serde_json::Value, RuntimeError>;

/// The result of an asynchronous operation.
pub type AsyncOpResult = Pin<Box<dyn Future<Output = OpResult> + Send>>;

/// Shared state handle for async ops.
pub type SharedOpState = Arc<Mutex<OpState>>;

/// Global monotonic op ID counter (shared across ALL constructors — B1 fix).
static NEXT_OP_ID: AtomicU16 = AtomicU16::new(1);

/// A sync operation function.
pub type SyncOp = Arc<dyn Fn(&mut OpState) -> OpResult + Send + Sync>;
/// An async operation function.
pub type AsyncOp = Arc<dyn Fn(SharedOpState) -> AsyncOpResult + Send + Sync>;
/// A sync operation with JSON input.
pub type SyncOpWithInput =
    Arc<dyn Fn(&mut OpState, serde_json::Value) -> OpResult + Send + Sync>;
/// An async operation with JSON input.
pub type AsyncOpWithInput = Arc<
    dyn Fn(SharedOpState, serde_json::Value) -> AsyncOpResult + Send + Sync,
>;

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

/// An operation declaration — the core abstraction for runtime extensions.
#[derive(Clone)]
pub struct OpDecl {
    /// Unique operation ID (auto-assigned from global counter).
    pub id: OpId,
    /// Human-readable name.
    pub name: &'static str,
    /// Whether this op is async.
    pub is_async: bool,
    /// The operation function.
    pub op_fn: OpFn,
}

impl OpDecl {
    fn next_id() -> OpId {
        NEXT_OP_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn new_sync(
        name: &'static str,
        f: impl Fn(&mut OpState) -> OpResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: Self::next_id(),
            name,
            is_async: false,
            op_fn: OpFn::Sync(Arc::new(f)),
        }
    }

    pub fn new_async(
        name: &'static str,
        f: impl Fn(SharedOpState) -> AsyncOpResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: Self::next_id(),
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
        Self {
            id: Self::next_id(),
            name,
            is_async: false,
            op_fn: OpFn::SyncWithInput(Arc::new(f)),
        }
    }

    pub fn new_async_with_input(
        name: &'static str,
        f: impl Fn(SharedOpState, serde_json::Value) -> AsyncOpResult
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            id: Self::next_id(),
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
                Err(RuntimeError::op_mismatch(self.name, "async"))
            }
            OpFn::SyncWithInput(f) => {
                f(state, input.unwrap_or(serde_json::Value::Null))
            }
            OpFn::AsyncWithInput(_) => {
                Err(RuntimeError::op_mismatch(self.name, "async"))
            }
        }
    }

    /// Execute the async operation. Takes the state as Arc<Mutex<OpState>>.
    pub fn execute_async(
        &self,
        state: SharedOpState,
        input: Option<serde_json::Value>,
    ) -> AsyncOpResult {
        match &self.op_fn {
            OpFn::Async(f) => f(state),
            OpFn::AsyncWithInput(f) => {
                f(state, input.unwrap_or(serde_json::Value::Null))
            }
            OpFn::Sync(_) | OpFn::SyncWithInput(_) => {
                let name = self.name;
                Box::pin(
                    async move { Err(RuntimeError::op_mismatch(name, "sync")) },
                )
            }
        }
    }
}

impl std::fmt::Debug for OpDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpDecl")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("async", &self.is_async)
            .finish()
    }
}
