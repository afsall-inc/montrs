// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError, FromEnv};
use ui::Button;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// [REQUIRED] 1. Define Application Error
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum MyAppError {
    #[error("Internal Error: {0}")]
    Internal(String),
}

// [REQUIRED] 2. Define Application Environment
#[derive(Clone)]
struct MyEnv;
impl EnvConfig for MyEnv {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        match key {
            "APP_ENV" => Ok("development".to_string()),
            _ => Err(EnvError::MissingKey(key.to_string())),
        }
    }
}

// [REQUIRED] 3. Define Application Configuration
#[derive(Clone)]
struct MyAppConfig;
impl AppConfig for MyAppConfig {
    type Error = MyAppError;
    type Env = MyEnv;
}

// [REQUIRED] 4. UI Components
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main class="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
            <h1 class="text-4xl font-bold mb-4">"MontRS Workspace"</h1>
            <Button on_click=move |_| set_count.update(|n| *n += 1)>
                "Count: " {count}
            </Button>
            <p class="mt-4 text-gray-400 text-sm">
                "Using shared UI from " <code class="bg-slate-800 px-1 rounded">"packages/ui"</code>
            </p>
        </main>
    }
}

// [REQUIRED] 5. Main Entry Point
fn main() {
    // [EXPLICIT] Manual bootstrapping
    let spec = AppSpec::new(MyAppConfig, MyEnv)
        .with_target(Target::Wasm);
    
    // [EXPLICIT] Explicit mount
    mount_to_body(|| view! { <App /> });
}

