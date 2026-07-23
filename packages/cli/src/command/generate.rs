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

use anyhow::{Result, anyhow};
use console::style;
use montrs_utils::{to_pascal_case, to_snake_case};
use std::{fs, path::Path};

pub async fn plate(name: String) -> Result<()> {
    let name_pascal = to_pascal_case(&name);
    let name_snake = to_snake_case(&name);

    println!(
        "{} Generating plate: {}",
        style("🔨").bold(),
        style(&name_pascal).cyan().bold()
    );

    let content = format!(
        r#"use montrs_core::{{Plate, PlateContext, Router, AppConfig}};
use async_trait::async_trait;

pub struct {name_pascal}Plate;

#[async_trait]
impl<C: AppConfig> Plate<C> for {name_pascal}Plate {{
    fn name(&self) -> &'static str {{
        "{name_snake}"
    }}

    fn dependencies(&self) -> Vec<&'static str> {{
        vec![]
    }}

    async fn init(&self, _ctx: &mut PlateContext<C>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {{
        Ok(())
    }}

    fn register_routes(&self, _router: &mut Router<C>) {{
        // Register routes here:
        // _router.register({name_pascal}Route);
    }}
}}
"#
    );

    let dir = Path::new("src/plates");
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    let file_path = dir.join(format!("{}.rs", name_snake));
    if file_path.exists() {
        return Err(anyhow!("Plate file already exists: {:?}", file_path));
    }

    fs::write(&file_path, content)?;

    println!(
        "{} Created plate at: {}",
        style("✨").green().bold(),
        style(file_path.display()).underlined()
    );
    println!(
        "Next steps:\n  1. Add `pub mod {};` to `src/plates/mod.rs`\n  2. \
         Register the plate in `src/main.rs` using \
         `.with_plate(Box::new({}Plate))`",
        name_snake, name_pascal
    );

    Ok(())
}

pub async fn route(path: String, plate: String) -> Result<()> {
    let plate_snake = to_snake_case(&plate);
    let route_name = path
        .replace('/', "_")
        .replace(':', "")
        .trim_matches('_')
        .to_string();
    let route_name_pascal = if route_name.is_empty() {
        "Index".to_string()
    } else {
        to_pascal_case(&route_name)
    };

    println!(
        "{} Generated route {} for plate {}",
        style("🛣️").bold(),
        style(&path).yellow().bold(),
        style(&plate).cyan().bold()
    );

    let content = format!(
        r#"use montrs_core::{{Route, RouteParams, RouteLoader, RouteAction, RouteView, RouteContext, RouteError, AppConfig}};
use async_trait::async_trait;
use leptos::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Serialize, Deserialize)]
pub struct {route_name_pascal}Params {{}}
impl RouteParams for {route_name_pascal}Params {{}}

pub struct {route_name_pascal}Loader;
#[async_trait]
impl<C: AppConfig> RouteLoader<{route_name_pascal}Params, C> for {route_name_pascal}Loader {{
    type Output = String;
    async fn load(&self, _ctx: RouteContext<'_, C>, _params: {route_name_pascal}Params) -> Result<Self::Output, RouteError> {{
        Ok("Hello from {route_name_pascal}Loader".to_string())
    }}
}}

pub struct {route_name_pascal}Action;
#[async_trait]
impl<C: AppConfig> RouteAction<{route_name_pascal}Params, C> for {route_name_pascal}Action {{
    type Input = String;
    type Output = String;
    async fn act(&self, _ctx: RouteContext<'_, C>, _params: {route_name_pascal}Params, input: Self::Input) -> Result<Self::Output, RouteError> {{
        Ok(format!("Echo: {{}}", input))
    }}
}}

pub struct {route_name_pascal}View;
impl RouteView for {route_name_pascal}View {{
    fn render(&self) -> impl IntoView {{
        view! {{ <div>"View for {path}"</div> }}
    }}
}}

pub struct {route_name_pascal}Route;
impl<C: AppConfig> Route<C> for {route_name_pascal}Route {{
    type Params = {route_name_pascal}Params;
    type Loader = {route_name_pascal}Loader;
    type Action = {route_name_pascal}Action;
    type View = {route_name_pascal}View;

    fn path() -> &'static str {{
        "{path}"
    }}
    fn loader(&self) -> Self::Loader {{ {route_name_pascal}Loader }}
    fn action(&self) -> Self::Action {{ {route_name_pascal}Action }}
    fn view(&self) -> Self::View {{ {route_name_pascal}View }}
}}
"#
    );

    let dir = Path::new("src/plates").join(&plate_snake).join("routes");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    let file_name = format!(
        "{}.rs",
        if route_name.is_empty() {
            "index".to_string()
        } else {
            route_name.to_lowercase()
        }
    );
    let file_path = dir.join(&file_name);

    if file_path.exists() {
        return Err(anyhow!("Route file already exists: {:?}", file_path));
    }

    fs::write(&file_path, content)?;

    println!(
        "{} Created route at: {}",
        style("✨").green().bold(),
        style(file_path.display()).underlined()
    );
    println!(
        "Next steps:\n  1. Add `pub mod {};` to `src/plates/{}/mod.rs`\n  2. \
         Register the route in `{}Plate::register_routes`",
        if route_name.is_empty() {
            "index".to_string()
        } else {
            route_name.to_lowercase()
        },
        plate_snake,
        to_pascal_case(&plate)
    );

    Ok(())
}

pub async fn haptics(name: String, _target: String) -> Result<()> {
    let name_pascal = to_pascal_case(&name);
    let name_snake = to_snake_case(&name);
    println!(
        "{} Generating haptics provider: {}",
        style("H").bold(),
        style(&name_pascal).cyan().bold()
    );
    let file_path = Path::new("src")
        .join("haptics")
        .join(format!("{}.rs", name_snake));
    if let Some(parent) = file_path.parent()
        && !parent.exists()
    {
        std::fs::create_dir_all(parent)?;
    }
    let content = format!(
        "use montrs_haptics::{{HapticsProvider, HapticsConfig, HapticsTarget, \
         ImpactStyle, create_haptics_provider}};

pub struct {0}Haptics {{
    provider: Box<dyn HapticsProvider>,
}}

impl {0}Haptics {{
    pub fn new() -> Self {{
        Self {{
            provider: create_haptics_provider(&HapticsConfig {{
                enabled: true,
                target: HapticsTarget::Desktop,
            }}),
        }}
    }}
    pub fn impact_light(&self) {{ self.provider.impact(ImpactStyle::Light); }}
    pub fn impact_medium(&self) {{ self.provider.impact(ImpactStyle::Medium); \
         }}
    pub fn impact_heavy(&self) {{ self.provider.impact(ImpactStyle::Heavy); }}
    pub fn selection(&self) {{ self.provider.selection_changed(); }}
    pub fn is_supported(&self) -> bool {{ self.provider.is_supported(); }}
}}
",
        name_pascal,
    );
    std::fs::write(&file_path, &content)?;
    println!(
        "{} Created haptics provider at: {}",
        style("").green().bold(),
        style(file_path.display()).underlined()
    );
    println!("Next steps:");
    println!("  1. Enable haptics feature (web, desktop, or mobile)");
    println!("  2. Use `{}Haptics::new()` in your plate", name_pascal);
    Ok(())
}

