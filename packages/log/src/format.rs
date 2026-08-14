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

//! Log format parsing and rendering.

use serde::{Deserialize, Serialize};

/// The on-disk / streaming format for a single log line.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum LogFormat {
    /// Plain text lines, one per message.
    #[default]
    Text,
    /// JSON object per line: `{"ts":..., "level":..., "msg":...}`.
    Json,
    /// logfmt: `ts=... level=info msg="..." field=value`.
    Logfmt,
}

impl LogFormat {
    /// Render a raw output line into a normalized structured record.
    pub fn render<'a>(
        &self,
        timestamp: &'a str,
        level: &'a str,
        service: &'a str,
        message: &'a str,
    ) -> String {
        match self {
            LogFormat::Text => format!("[{timestamp}] [{level}] {service}: {message}"),
            LogFormat::Json => {
                let rec = StructuredLog {
                    ts: timestamp.to_string(),
                    level: level.to_string(),
                    service: service.to_string(),
                    msg: message.to_string(),
                };
                serde_json::to_string(&rec).unwrap_or_else(|_| message.to_string())
            }
            LogFormat::Logfmt => {
                format!(
                    "ts={} level={} service={} msg=\"{}\"",
                    timestamp,
                    level,
                    service,
                    message.replace('"', "\\\"")
                )
            }
        }
    }

    /// Parse a JSON-structured log line into a record, if it is one.
    pub fn parse_json(line: &str) -> Option<StructuredLog> {
        serde_json::from_str::<StructuredLog>(line).ok()
    }
}

/// A structured (JSON) log record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLog {
    pub ts: String,
    pub level: String,
    pub service: String,
    pub msg: String,
}