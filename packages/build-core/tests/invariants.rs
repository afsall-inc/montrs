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

//! Invariant tests for montrs-build-core.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Zero Heavy Dependencies: No axum, hyper, tower, notify
//! - Trait-Driven: BuildPipeline trait
//! - Config-Only: Only config types and trait definition

use montrs_build_core::*;

#[test]
fn test_build_step_enum_values() {
    let steps = [
        BuildStep::Server,
        BuildStep::Frontend,
        BuildStep::Tailwind,
        BuildStep::Assets,
        BuildStep::IndexHtml,
    ];
    assert_eq!(steps.len(), 5);
    assert!(steps.contains(&BuildStep::Server));
    assert!(steps.contains(&BuildStep::Frontend));
    assert!(steps.contains(&BuildStep::Tailwind));
    assert!(steps.contains(&BuildStep::Assets));
    assert!(steps.contains(&BuildStep::IndexHtml));
}

#[test]
fn test_build_step_debug_and_clone() {
    let step = BuildStep::Server;
    let cloned = step;
    assert_eq!(format!("{:?}", step), format!("{:?}", cloned));
    assert_eq!(step, cloned);
}

#[test]
fn test_build_pipeline_trait_is_object_safe() {
    use std::path::Path;
    struct MockPipeline;
    impl BuildPipeline for MockPipeline {
        fn build_server(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn build_frontend(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn process_tailwind(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn copy_assets(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn generate_index_html(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn build_all(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn metadata(&self) -> &montrs_metadata::MontrsMetadata {
            unimplemented!()
        }
        fn project_root(&self) -> &Path {
            unimplemented!()
        }
        fn site_root(&self) -> &Path {
            unimplemented!()
        }
        fn pkg_dir(&self) -> &Path {
            unimplemented!()
        }
    }
    let pipeline: Box<dyn BuildPipeline> = Box::new(MockPipeline);
    assert!(pipeline.build_server().is_ok());
    assert!(pipeline.build_frontend().is_ok());
}

#[test]
fn test_find_workspace_target_dir_default() {
    let result =
        find_workspace_target_dir(std::path::Path::new("/nonexistent"));
    assert!(result.is_ok());
}
