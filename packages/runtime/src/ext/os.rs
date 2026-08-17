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

//! OS extension — environment variables, exit, hostname, loadavg, uptime.

use crate::{
    RuntimeExtension, error::RuntimeError, ops::OpDecl,
    permissions::Permissions, type_map::OpState,
};

pub fn init() -> RuntimeExtension {
    RuntimeExtension::builder("os")
        .ops(vec![
            OpDecl::new_sync_with_input(
                "os.env_get",
                |state: &mut OpState, input: serde_json::Value| {
                    let key = input.as_str().unwrap_or("").to_string();
                    state
                        .get::<Permissions>()
                        .ok_or_else(|| {
                            RuntimeError::internal("no permissions")
                        })?
                        .check_env(&key)?;
                    Ok(serde_json::json!(std::env::var(&key).ok()))
                },
            ),
            OpDecl::new_sync("os.env_vars", |state: &mut OpState| {
                state
                    .get::<Permissions>()
                    .ok_or_else(|| RuntimeError::internal("no permissions"))?
                    .check_sys()?;
                let vars: std::collections::HashMap<String, String> =
                    std::env::vars().collect();
                Ok(serde_json::json!(vars))
            }),
            OpDecl::new_sync_with_input(
                "os.exit",
                |_state: &mut OpState, input: serde_json::Value| {
                    let code = input.as_i64().unwrap_or(0) as i32;
                    std::process::exit(code);
                },
            ),
            OpDecl::new_sync("os.hostname", |_state: &mut OpState| {
                let hostname = std::env::var("HOSTNAME")
                    .or_else(|_| std::env::var("COMPUTERNAME"))
                    .unwrap_or_else(|_| "unknown".to_string());
                Ok(serde_json::json!(hostname))
            }),
            OpDecl::new_sync("os.uptime", |_state: &mut OpState| {
                Ok(serde_json::json!(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs_f64())
                        .unwrap_or(0.0)
                ))
            }),
            OpDecl::new_sync("os.loadavg", |_state: &mut OpState| {
                #[cfg(target_os = "linux")]
                {
                    if let Ok(content) =
                        std::fs::read_to_string("/proc/loadavg")
                    {
                        if let Some(parts) = content.split_whitespace().next() {
                            return Ok(serde_json::json!(parts));
                        }
                    }
                }
                Ok(serde_json::json!(null))
            }),
        ])
        .build()
}
