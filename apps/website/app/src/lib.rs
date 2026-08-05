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
        .with_target(Target::Wasm)
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
