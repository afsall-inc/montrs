use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError, FromEnv};
use tailwind_fuse::*;
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
        // [EXPLICIT] Explicitly handle environment variables
        match key {
            "APP_NAME" => Ok("MontRS Default App".to_string()),
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

// [OPTIONAL] 4. Styling with tailwind-fuse
#[derive(TwClass)]
#[tw(class = "px-6 py-2 rounded-lg transition-colors")]
struct CounterBtn {
    variant: BtnVariant,
}

#[derive(TwVariant)]
enum BtnVariant {
    #[tw(default, class = "bg-blue-600 hover:bg-blue-500 text-white")]
    Primary,
    #[tw(class = "bg-gray-600 hover:bg-gray-500 text-white")]
    Secondary,
}

// [REQUIRED] 5. Root View
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let btn_class = CounterBtn { variant: BtnVariant::Primary };

    view! {
        <main class="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
            <h1 class="text-4xl font-bold mb-4">"Built with MontRS & Leptos"</h1>
            <button
                class=btn_class.to_class()
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Count: " {count}
            </button>
            <p class="mt-4 text-gray-400 text-sm">
                "Powered by " <code class="bg-slate-800 px-1 rounded">"tailwind-fuse"</code>
            </p>
        </main>
    }
}

// [REQUIRED] 6. Main Entry Point
fn main() {
    // [EXPLICIT] Manual bootstrapping of AppSpec
    let spec = AppSpec::new(MyAppConfig, MyEnv)
        .with_target(Target::Wasm);
    
    // [EXPLICIT] Explicit mounting to the DOM
    // In a real MontRS app, we'd use spec.boot() which handles SSR/Hydration
    // but for a simple Wasm mount, we can use this:
    mount_to_body(|| view! { <App /> });
}
