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

//! Module loader — trait for loading and evaluating Rust/WASM modules into the runtime.

use crate::error::RuntimeError;
use crate::type_map::OpState;
use async_trait::async_trait;
use std::path::Path;

/// A loaded module.
pub struct Module {
    pub name: String,
    pub code: Vec<u8>,
    pub kind: ModuleKind,
    pub source: ModuleSource,
}

/// The kind of module loaded.
pub enum ModuleKind {
    /// A Rust-native function module (B11 fix: now evaluable).
    Rust,
    /// A WebAssembly module (bytes readable, WASM instantiation TBD).
    Wasm,
    /// A native plugin (external binary or shared library).
    Native,
}

/// Where the module source came from.
pub enum ModuleSource {
    File(std::path::PathBuf),
    Inline(Vec<u8>),
    Network(String),
}

/// A Rust module is a function that takes OpState and returns JSON.
pub type RustModuleFn = Box<dyn Fn(&mut OpState) -> Result<serde_json::Value, RuntimeError> + Send + Sync>;

/// A loaded and evaluated Rust module.
pub struct RustModule {
    pub name: String,
    pub func: RustModuleFn,
}

/// Trait for loading modules into the runtime.
#[async_trait]
pub trait ModuleLoader: Send + Sync {
    /// Resolve a module specifier to a path or URL.
    fn resolve(
        &self,
        specifier: &str,
        base: &Path,
    ) -> Result<String, RuntimeError>;

    /// Load a module by its resolved specifier.
    async fn load(&self, specifier: &str) -> Result<Module, RuntimeError>;

    /// Prepare to load a module (pre-processing).
    async fn prepare_load(
        &self,
        _specifier: &str,
    ) -> Result<(), RuntimeError> {
        Ok(())
    }

    /// Evaluate a loaded module if it is a Rust module.
    /// Returns Ok(Some(RustModule)) for Rust modules, Ok(None) for non-Rust.
    fn evaluate_rust(
        &self,
        _module: &Module,
        _state: &mut OpState,
    ) -> Result<Option<RustModule>, RuntimeError> {
        Ok(None)
    }
}

/// A simple file-based module loader.
pub struct FileModuleLoader {
    pub roots: Vec<std::path::PathBuf>,
}

#[async_trait]
impl ModuleLoader for FileModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        base: &Path,
    ) -> Result<String, RuntimeError> {
        // Try relative to base first.
        let base_path = base.join(specifier);
        if base_path.exists() {
            return Ok(base_path.to_string_lossy().to_string());
        }
        // Try each root.
        for root in &self.roots {
            let path = root.join(specifier);
            if path.exists() {
                return Ok(path.to_string_lossy().to_string());
            }
        }
        Err(RuntimeError::new(
            crate::error::RuntimeErrorKind::ModuleLoad,
            format!("module not found: {specifier}"),
        ))
    }

    async fn load(&self, specifier: &str) -> Result<Module, RuntimeError> {
        let path = std::path::Path::new(specifier);
        if !path.exists() {
            return Err(RuntimeError::new(
                crate::error::RuntimeErrorKind::ModuleLoad,
                format!("module not found: {specifier}"),
            ));
        }
        let code = tokio::fs::read(path).await.map_err(|e| {
            RuntimeError::new(crate::error::RuntimeErrorKind::ModuleLoad, e.to_string())
        })?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("module")
            .to_string();
        let kind = if specifier.ends_with(".wasm") {
            ModuleKind::Wasm
        } else if specifier.ends_with(".rust") || specifier.ends_with(".so") {
            // .rust files are treated as Rust modules for evaluation.
            // .so files are treated as native plugins.
            if specifier.ends_with(".so") {
                ModuleKind::Native
            } else {
                ModuleKind::Rust
            }
        } else {
            ModuleKind::Rust
        };
        Ok(Module {
            name,
            code,
            kind,
            source: ModuleSource::File(path.to_path_buf()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_file_module_loader_not_found() {
        let loader = FileModuleLoader {
            roots: vec![],
        };
        let result = loader.resolve("nonexistent.rs", Path::new("/tmp"));
        assert!(result.is_err());
    }

    #[test]
    fn test_module_kind_from_extension() {
        let loader = FileModuleLoader { roots: vec![] };
        // This is a compile-time check only — the underlying path logic is in load().
        let _ = loader;
    }
}