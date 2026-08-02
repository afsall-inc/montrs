//! MontRS website — montrs.com

pub mod pages;
pub mod routes;
pub mod components;
pub mod blocks;

use leptos::prelude::*;
use montrs_core::*;
use montrs_ui::prelude::*;

use crate::components::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <leptos_meta::Html attr:lang="en" />
            <leptos_meta::Meta charset="utf-8" />
            <leptos_meta::Meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <leptos_meta::Meta name="description" content="MontRS — A full-stack Rust framework for humans and agents" />
            <leptos_meta::Title text="MontRS" />
            <leptos_meta::Stylesheet id="leptos" href="/pkg/website.css" />

            <Header />
            <main class="min-h-screen">
                {RouterOutlet::<MyConfig>()}
            </main>
            <Footer />
        </ThemeProvider>
    }
}

// ---------------------------------------------------------------------------
// AppConfig + AppSpec
// ---------------------------------------------------------------------------

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

fn main() {
    AppSpec::new(MyConfig, MyEnv)
        .with_target(Target::Wasm)
        .with_plate(WebsitePlate)
        .mount_with_router(|| view! { <App /> });
}

// Re-export for server
pub use crate::routes::WebsitePlate;