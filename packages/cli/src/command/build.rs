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

use montrs_build::{BuildPipeline, Pipeline};
use montrs_metadata::{DeployMode, DeployTarget};
use std::path::Path;

/// Resolve the build mode.
///
/// Precedence: `--dev` flag > `MONTRS_MODE` env > `[deploy] mode` > production.
/// Returns `(mode, is_release)`.
fn resolve_build_mode(
    pipeline: &Pipeline,
    dev_flag: bool,
    release_flag: bool,
) -> anyhow::Result<(DeployMode, bool)> {
    if dev_flag && release_flag {
        anyhow::bail!("--dev and --release are mutually exclusive");
    }

    let mode = if dev_flag {
        DeployMode::Development
    } else if let Ok(value) = std::env::var("MONTRS_MODE") {
        DeployMode::parse(&value).ok_or_else(|| {
            anyhow::anyhow!(
                "invalid MONTRS_MODE {value:?}: expected development/dev or \
                 production/prod"
            )
        })?
    } else {
        pipeline.meta.deploy.mode
    };

    // A production build must not silently ship the dev overlay. Require an
    // explicit `--dev` to build in development mode.
    if mode == DeployMode::Development && !dev_flag {
        anyhow::bail!(
            "refusing to build in development mode: [deploy] mode = \
             \"development\" (or MONTRS_MODE=development). Pass `--dev` to \
             build deliberately, or set mode = \"production\"."
        );
    }

    let is_release = release_flag || mode == DeployMode::Production;
    Ok((mode, is_release))
}

/// Only the `ssr` deployment target is implemented today.
fn check_target(target: DeployTarget) -> anyhow::Result<()> {
    if target != DeployTarget::Ssr {
        anyhow::bail!(
            "[deploy] target = \"{}\" is not implemented yet; only \"ssr\" is \
             supported (static export, desktop, and mobile targets are \
             planned).",
            target.as_str()
        );
    }
    Ok(())
}

pub async fn run(dev: bool) -> anyhow::Result<()> {
    let mut pipeline = Pipeline::from_root(Path::new("."))?;
    check_target(pipeline.meta.deploy.target)?;
    let (mode, is_release) =
        resolve_build_mode(&pipeline, dev, crate::config::current_release())?;
    pipeline.release = is_release;
    println!("Building in {} mode", mode.as_str());
    crate::command::resolve_pipeline_bins(&mut pipeline);
    pipeline.build_all()
}
