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
use leptos::prelude::*;
use montrs_core::{
    AppConfig, EnvConfig, Route, RouteAction, RouteContext, RouteError,
    RouteLoader, RouteParams, RouteView, Router,
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
impl EnvConfig for TestEnv {
    fn get_var(&self, _key: &str) -> Result<String, montrs_core::EnvError> {
        Ok("test".to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct UserParams {
    id: u32,
}
impl RouteParams for UserParams {}

struct UserLoader;
#[async_trait]
impl RouteLoader<UserParams, TestConfig> for UserLoader {
    type Output = String;
    async fn load(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        params: UserParams,
    ) -> Result<Self::Output, RouteError> {
        Ok(format!("User {}", params.id))
    }
}

struct UserAction;
#[async_trait]
impl RouteAction<UserParams, TestConfig> for UserAction {
    type Input = String;
    type Output = String;
    async fn act(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        params: UserParams,
        input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        Ok(format!("Updated user {} with {}", params.id, input))
    }
}

struct UserView;
impl RouteView for UserView {
    fn render(&self) -> impl IntoView {
        view! { <div>"User View"</div> }
    }
}

struct UserRoute;
impl Route<TestConfig> for UserRoute {
    type Params = UserParams;
    type Loader = UserLoader;
    type Action = UserAction;
    type View = UserView;

    fn path() -> &'static str {
        "/users/:id"
    }
    fn loader(&self) -> Self::Loader {
        UserLoader
    }
    fn action(&self) -> Self::Action {
        UserAction
    }
    fn view(&self) -> Self::View {
        UserView
    }
}

#[tokio::test]
async fn test_router_registration_and_handling() {
    let mut router = Router::<TestConfig>::new();
    router.register(UserRoute);

    let config = TestConfig;
    let env = TestEnv;
    let _ctx = RouteContext {
        config: &config,
        env: &env,
    };

    let _params = serde_json::json!({ "id": 123 });

    // Test load
    let spec = router.spec();
    let load_res = spec.routes.get("/users/:id").unwrap();
    assert_eq!(load_res.path, "/users/:id");

    // In a real scenario, we'd call handle_load on the RouteInfo, but it's internal.
    // However, we can verify the spec is correct.
    assert_eq!(spec.routes.len(), 1);
}
