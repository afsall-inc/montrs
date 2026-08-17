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

//! MontRS website — montrs.com

#![recursion_limit = "512"]

pub mod blocks;
pub mod components;
pub mod pages;
pub mod routes;

use crate::{components::*, routes::*};
use leptos::prelude::*;
use montrs_core::{
    AppConfig, AppSpec, EnvConfig, EnvError, Plate, RouterOutlet, Target,
};
use montrs_ui::prelude::*;

pub fn build_spec() -> AppSpec<MyConfig> {
    let mut spec = AppSpec::new(MyConfig, MyEnv)
        .with_target(Target::Web)
        .with_plate(WebsitePlate);
    WebsitePlate.register_routes(&mut spec.router);
    spec
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    let spec = build_spec();
    leptos::mount::hydrate_body(move || {
        provide_context(spec.router);
        App()
    });
}

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <leptos_router::components::Router>
            <ThemeProvider>
                <Header />
                <main class="min-h-screen">
                    {RouterOutlet::<MyConfig>()}
                </main>
                <Footer />
            </ThemeProvider>
        </leptos_router::components::Router>
    }
}

#[derive(Clone)]
pub struct MyEnv;

impl EnvConfig for MyEnv {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        match key {
            "APP_NAME" => Ok("montrs-website".to_string()),
            _ => Err(EnvError::MissingKey(key.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MyAppError {
    Internal(String),
}

impl std::fmt::Display for MyAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyAppError::Internal(msg) => write!(f, "Internal: {}", msg),
        }
    }
}

impl std::error::Error for MyAppError {}

#[derive(Clone)]
pub struct MyConfig;

impl AppConfig for MyConfig {
    type Error = MyAppError;
    type Env = MyEnv;
}
