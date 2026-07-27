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
