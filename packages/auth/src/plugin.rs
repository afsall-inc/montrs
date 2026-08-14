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

//! Auth plugin system — compose only the features you need.

use crate::context::AuthState;
use crate::AuthError;
use axum::extract::Request;
use axum::response::Response;
use axum::Router;

/// Schema extension declared by a plugin (for OpenAPI / migrations).
#[derive(Debug, Clone)]
pub struct SchemaExtension {
    pub table: String,
    pub description: String,
}

/// The auth plugin trait. Implement this to add auth features.
pub trait AuthPlugin: Send + Sync + 'static {
    /// A short name identifying the plugin.
    fn name(&self) -> &'static str;

    /// Called once when [`crate::MontrsAuth`] is built. Store `AuthState` if needed.
    fn on_build(&mut self, _state: &AuthState) -> Result<(), AuthError> {
        Ok(())
    }

    /// The axum router this plugin registers.
    fn router(&self) -> Router {
        Router::new()
    }

    /// Hooks run before a request is handled.
    fn before_request(&self, _req: &Request) -> Result<(), AuthError> {
        Ok(())
    }

    /// Hooks run after a request is handled.
    fn after_request(&self, _resp: &Response) {}

    /// Optional schema extensions for docs / migrations.
    fn schema_extensions(&self) -> Vec<SchemaExtension> {
        Vec::new()
    }
}

/// Attach a plugin's router to an existing router.
pub fn mount_plugin(router: Router, plugin: &dyn AuthPlugin) -> Router {
    router.merge(plugin.router())
}
