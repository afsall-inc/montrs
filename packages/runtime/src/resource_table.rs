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

//! Resource table — typed handles for runtime resources (files, sockets, etc.).
//!
//! Inspired by Deno's `deno_core::ResourceTable`. Each resource is identified
//! by a `ResourceId` and can be any type implementing `Resource`.

use crate::error::RuntimeError;
use std::{
    any::Any,
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

/// A unique resource identifier.
pub type ResourceId = u32;

/// Trait for resources stored in the resource table.
pub trait Resource: Any + Send + Sync {
    fn name(&self) -> &str;
    /// Close the resource. Return an error if closing fails (B12 fix).
    fn close(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

/// The global resource table — stores typed handles on behalf of extensions.
#[derive(Default)]
pub struct ResourceTable {
    next_id: AtomicU32,
    resources: HashMap<ResourceId, Box<dyn Resource>>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU32::new(1),
            resources: HashMap::new(),
        }
    }

    /// Add a resource, returning its ID.
    pub fn add(&mut self, resource: Box<dyn Resource>) -> ResourceId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.resources.insert(id, resource);
        id
    }

    /// Get a reference to a resource by ID.
    pub fn get(&self, id: ResourceId) -> Option<&dyn Resource> {
        self.resources.get(&id).map(|r| r.as_ref())
    }

    /// Get a mutable reference to a resource by ID.
    pub fn get_mut(&mut self, id: ResourceId) -> Option<&mut dyn Resource> {
        self.resources.get_mut(&id).map(|r| r.as_mut())
    }

    /// Get a resource downcasted to a specific type.
    pub fn get_typed<T: Resource>(&self, id: ResourceId) -> Option<&T> {
        self.resources
            .get(&id)
            .and_then(|r| r.as_any().downcast_ref::<T>())
    }

    /// Get a resource downcasted to a specific type (mutable).
    pub fn get_typed_mut<T: Resource>(
        &mut self,
        id: ResourceId,
    ) -> Option<&mut T> {
        self.resources
            .get_mut(&id)
            .and_then(|r| r.as_mut_any().downcast_mut::<T>())
    }

    /// Take a resource by ID (removes it from the table).
    pub fn take(&mut self, id: ResourceId) -> Option<Box<dyn Resource>> {
        self.resources.remove(&id)
    }

    /// Close a resource (removes and calls close handler). Errors propagate (B12 fix).
    pub fn close(&mut self, id: ResourceId) -> Result<(), RuntimeError> {
        if let Some(resource) = self.resources.remove(&id) {
            resource.close()?;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
    pub fn clear(&mut self) {
        self.resources.clear();
    }

    pub fn ids(&self) -> Vec<ResourceId> {
        self.resources.keys().copied().collect()
    }
}

impl dyn Resource {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
    pub fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A simple file resource.
pub struct FileResource {
    pub path: String,
    pub file: tokio::fs::File,
}

impl Resource for FileResource {
    fn name(&self) -> &str {
        "file"
    }
}

/// A simple TCP stream resource.
pub struct TcpStreamResource {
    pub stream: tokio::net::TcpStream,
}

impl Resource for TcpStreamResource {
    fn name(&self) -> &str {
        "tcp_stream"
    }
}
