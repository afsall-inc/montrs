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

use crate::config::MontrsConfig;
use montrs_runner::TaskRunner;
use std::path::Path;

pub async fn run(
    task_name: String,
    affected: bool,
    since: Option<String>,
    no_cache: bool,
) -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner =
        TaskRunner::from_config_tasks(config.meta.tasks, Path::new("."));

    if no_cache {
        // SAFETY: set before any task executes; the process is single-threaded
        // at this point on the CLI's startup path.
        unsafe { std::env::set_var("MONTRS_NO_CACHE", "1") };
    }

    if affected {
        let changed =
            montrs_scm::changed_files(Path::new("."), since.as_deref());
        if changed.is_empty() {
            println!("No affected files — nothing to run.");
            return Ok(());
        }
        println!(
            "Running tasks affected by {} changed file(s).",
            changed.len()
        );
        return runner
            .run_filtered(&task_name, |t| {
                montrs_scm::matches_any(&changed, &t.sources)
            })
            .await;
    }

    runner.run(&task_name).await
}

pub async fn list() -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;
    let runner =
        TaskRunner::from_config_tasks(config.meta.tasks, Path::new("."));
    runner.list()?;
    Ok(())
}
