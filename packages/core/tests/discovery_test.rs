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

use async_trait::async_trait;
use montrs_core::{
    ActionResponse, AppConfig, LoaderResponse, RouteAction, RouteContext,
    RouteError, RouteLoader, RouteParams,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct TestConfig;
impl AppConfig for TestConfig {
    type Error = std::io::Error;
    type Env = TestEnv;
}

#[derive(Clone)]
struct TestEnv;
impl montrs_core::EnvConfig for TestEnv {
    fn get_var(&self, _key: &str) -> Result<String, montrs_core::EnvError> {
        Ok("test".to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct EmptyParams;
impl RouteParams for EmptyParams {}

struct TestLoader;

#[async_trait]
impl RouteLoader<EmptyParams, TestConfig> for TestLoader {
    type Output = LoaderResponse;

    async fn load(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        _params: EmptyParams,
    ) -> Result<Self::Output, RouteError> {
        Ok(LoaderResponse {
            data: serde_json::json!({}),
        })
    }

    fn description(&self) -> &'static str {
        "A test loader for discovery verification"
    }
}

struct TestAction;

#[async_trait]
impl RouteAction<EmptyParams, TestConfig> for TestAction {
    type Input = serde_json::Value;
    type Output = ActionResponse;

    async fn act(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        _params: EmptyParams,
        _input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        Ok(ActionResponse {
            data: serde_json::json!({}),
        })
    }

    fn description(&self) -> &'static str {
        "A test action for discovery verification"
    }
}

#[tokio::test]
async fn test_discovery_types_compile() {
    let _loader = TestLoader;
    let _action = TestAction;
    let _params = EmptyParams;
    let _config = TestConfig;
    let _env = TestEnv;
}

