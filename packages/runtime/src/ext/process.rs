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

//! Process extension — run command with output.

use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::ops::{self, OpDecl};
use crate::permissions::Permissions;
use crate::RuntimeExtension;
use tokio::process::Command;

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("process")
        .ops(vec![
            OpDecl::new_async_with_input("process.run", |state: ops::SharedOpState, input: serde_json::Value| {
                let cmd = input.get("cmd").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let args: Vec<String> = input.get("args")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                Box::pin(async move {
                    let perms = state.lock().await.get::<Permissions>().cloned();
                    if let Some(p) = perms {
                        p.check_run(&cmd)?;
                    }
                    #[cfg(unix)]
                    let output = Command::new("sh").arg("-c").arg(&cmd).args(&args).output().await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, format!("process.run {cmd}: {e}"))
                    })?;
                    #[cfg(not(unix))]
                    let output = Command::new("cmd").arg("/C").arg(&cmd).args(&args).output().await.map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::OpExecution, format!("process.run {cmd}: {e}"))
                    })?;
                    Ok(serde_json::json!({
                        "status": output.status.code(),
                        "success": output.status.success(),
                        "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
                        "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
                    }))
                })
            }),
        ])
        .build()
}