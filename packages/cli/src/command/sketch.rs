use anyhow::{Result, anyhow};
use console::style;
use montrs_utils::{to_pascal_case, to_snake_case};
use std::fs;
use std::path::Path;

pub async fn run(name: String, kind: String) -> Result<()> {
    println!(
        "{} Sketching {} component: {}",
        style("✏️").bold(),
        style(&kind).yellow(),
        style(&name).cyan().bold()
    );

    let content = match kind.as_str() {
        "plate" => generate_plate_sketch(&name),
        "route" => generate_route_sketch(&name),
        "app" => generate_app_sketch(&name),
        _ => {
            return Err(anyhow!(
                "Unknown component kind: {}. Use plate, route, or app.",
                kind
            ));
        }
    };

    let file_name = format!("{}.sketch.rs", name.to_lowercase().replace(' ', "_"));
    let file_path = Path::new(&file_name);

    if file_path.exists() {
        return Err(anyhow!("Sketch file already exists: {:?}", file_path));
    }

    fs::write(file_path, content)?;

    println!(
        "{} Created sketch at: {}",
        style("✨").green().bold(),
        style(file_path.display()).underlined()
    );
    println!(
        "\nThis is a 'Scaffolded Explicit' sketch. You can edit it freely.\nRun `montrs expand {}` to convert it into a full project structure.",
        file_path.display()
    );

    Ok(())
}

fn generate_plate_sketch(name: &str) -> String {
    let pascal = to_pascal_case(name);
    let snake = to_snake_case(name);
    format!(
        r#"//! MontRS Plate Sketch: {name}
//! This file contains an explicit, deterministic, single-file plate definition.
//! Run `montrs expand` to move this into src/plates/{snake}.rs

use montrs_core::{{Plate, PlateContext, Router, AppConfig}};
use async_trait::async_trait;

// [REQUIRED] Plate structure
pub struct {pascal}Plate;

#[async_trait]
impl<C: AppConfig> Plate<C> for {pascal}Plate {{
    fn name(&self) -> &'static str {{
        "{snake}"
    }}

    // [OPTIONAL] Explicitly declare dependencies for architectural integrity
    fn dependencies(&self) -> Vec<&'static str> {{
        vec![]
    }}

    // [OPTIONAL] Initialization logic
    async fn init(&self, _ctx: &mut PlateContext<C>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {{
        println!("Initializing {pascal}Plate");
        Ok(())
    }}

    // [REQUIRED] Route registration
    fn register_routes(&self, _router: &mut Router<C>) {{
        // _router.register({pascal}Route);
    }}
}}

// [OPTIONAL] Example Route inside sketch
// pub struct {pascal}Route;
// ... (implement Route trait explicitly here)
"#
    )
}

fn generate_route_sketch(name: &str) -> String {
    let pascal = to_pascal_case(name);
    format!(
        r#"//! MontRS Route Sketch: {name}
//! This file contains an explicit, deterministic, single-file route definition.

use montrs_core::{{Route, RouteParams, RouteLoader, RouteAction, RouteView, RouteContext, RouteError, AppConfig}};
use async_trait::async_trait;
use leptos::prelude::*;
use serde::{{Deserialize, Serialize}};

// [REQUIRED] Parameters (explicitly defined)
#[derive(Serialize, Deserialize)]
pub struct {pascal}Params {{
    pub id: Option<String>,
}}
impl RouteParams for {pascal}Params {{}}

// [REQUIRED] Data Loader (explicitly defined)
pub struct {pascal}Loader;
#[async_trait]
impl<C: AppConfig> RouteLoader<{pascal}Params, C> for {pascal}Loader {{
    type Output = String;
    async fn load(&self, _ctx: RouteContext<'_, C>, _params: {pascal}Params) -> Result<Self::Output, RouteError> {{
        Ok(format!("Loaded data for {pascal}"))
    }}
}}

// [OPTIONAL] Data Action (explicitly defined)
pub struct {pascal}Action;
#[async_trait]
impl<C: AppConfig> RouteAction<{pascal}Params, C> for {pascal}Action {{
    type Input = String;
    type Output = String;
    async fn act(&self, _ctx: RouteContext<'_, C>, _params: {pascal}Params, input: Self::Input) -> Result<Self::Output, RouteError> {{
        Ok(format!("Processed: {{}}", input))
    }}
}}

// [REQUIRED] View (explicitly defined)
pub struct {pascal}View;
impl RouteView for {pascal}View {{
    fn render(&self) -> impl IntoView {{
        view! {{ 
            <div class="p-4">
                <h1 class="text-xl font-bold">"{pascal} View"</h1>
                <p>"Explicitly scaffolded route view."</p>
            </div> 
        }}
    }}
}}

// [REQUIRED] Unified Route Trait
pub struct {pascal}Route;
impl<C: AppConfig> Route<C> for {pascal}Route {{
    type Params = {pascal}Params;
    type Loader = {pascal}Loader;
    type Action = {pascal}Action;
    type View = {pascal}View;

    fn path() -> &'static str {{ "/{name}" }}
    fn loader(&self) -> Self::Loader {{ {pascal}Loader }}
    fn action(&self) -> Self::Action {{ {pascal}Action }}
    fn view(&self) -> Self::View {{ {pascal}View }}
}}
"#
    )
}

fn generate_app_sketch(name: &str) -> String {
    let pascal = to_pascal_case(name);
    format!(
        r#"//! MontRS App Sketch: {name}
//! A complete, single-file application blueprint.

use montrs_core::{{AppConfig, AppSpec, EnvConfig, Target}};
use serde::{{Deserialize, Serialize}};
use thiserror::Error;

// 1. Define App Error
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum {pascal}Error {{
    #[error("Internal Error: {{0}}")]
    Internal(String),
}}

// 2. Define App Environment
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct {pascal}Env {{
    pub database_url: String,
}}
impl EnvConfig for {pascal}Env {{
    fn get_var(&self, key: &str) -> Result<String, montrs_core::EnvError> {{
        match key {{
            "DATABASE_URL" => Ok(self.database_url.clone()),
            _ => Err(montrs_core::EnvError::MissingKey(key.to_string())),
        }}
    }}
}}

// 3. Define App Configuration
#[derive(Clone)]
pub struct {pascal}Config;
impl AppConfig for {pascal}Config {{
    type Error = {pascal}Error;
    type Env = {pascal}Env;
}}

// 4. Entry point (Explicit Bootstrapping)
pub fn create_app() -> AppSpec<{pascal}Config> {{
    let config = {pascal}Config;
    let env = {pascal}Env::default();
    
    AppSpec::new(config, env)
        .with_target(Target::Server)
        // .with_plate(Box::new(MyPlate))
}}
"#
    )
}
