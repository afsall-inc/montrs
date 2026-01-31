//! todo-example: A comprehensive example demonstrating MontRS features.
//! This application integrates signals, schema validation, and the ORM layer
//! to build a simple but functional Todo management system.

use leptos::prelude::*;
use montrs_core::{
    AppConfig, AppSpec, Plate, PlateContext, Route, RouteAction, RouteContext, RouteError,
    RouteLoader, RouteParams, RouteView, Router, Target,
};
use montrs_orm::{DbBackend, FromRow, SqliteBackend};
use montrs_validator::Validator;
use serde::{Deserialize, Serialize};

// [REQUIRED] 1. Define the Application Error Type.
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum MyError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Generic error: {0}")]
    Generic(String),
}

// [REQUIRED] 2. Define the Application Environment.
#[derive(Clone)]
pub struct MyEnv;
impl montrs_core::EnvConfig for MyEnv {
    fn get_var(&self, key: &str) -> Result<String, montrs_core::EnvError> {
        match key {
            "DATABASE_URL" => Ok("sqlite::memory:".to_string()),
            _ => Err(montrs_core::EnvError::MissingKey(key.to_string())),
        }
    }
}

// [REQUIRED] 3. Define the Application Configuration.
#[derive(Clone)]
pub struct MyConfig {
    pub db_url: String,
}
impl AppConfig for MyConfig {
    type Error = MyError;
    type Env = MyEnv;
}

// [OPTIONAL] 4. Data Models & Validation
#[derive(Debug, Clone, Serialize, Deserialize, Validator)]
pub struct CreateTodo {
    #[validator(min_len = 3)]
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

// [REQUIRED] 5. Define explicit Route components
#[derive(Serialize, Deserialize)]
pub struct TodoParams {}
impl RouteParams for TodoParams {}

pub struct TodoLoader;
#[async_trait::async_trait]
impl RouteLoader<TodoParams, MyConfig> for TodoLoader {
    type Output = Vec<Todo>;
    async fn load(
        &self,
        _ctx: RouteContext<'_, MyConfig>,
        _params: TodoParams,
    ) -> Result<Self::Output, RouteError> {
        Ok(vec![])
    }
}

pub struct TodoAction;
#[async_trait::async_trait]
impl RouteAction<TodoParams, MyConfig> for TodoAction {
    type Input = CreateTodo;
    type Output = Todo;
    async fn act(
        &self,
        _ctx: RouteContext<'_, MyConfig>,
        _params: TodoParams,
        _input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        Ok(Todo {
            id: 1,
            title: "New Todo".to_string(),
            completed: false,
        })
    }
}

pub struct TodoViewImpl;
impl RouteView for TodoViewImpl {
    fn render(&self) -> impl IntoView {
        view! { <TodoApp /> }
    }
}

// [REQUIRED] 6. Unified Route Trait
pub struct TodoRoute;
impl Route<MyConfig> for TodoRoute {
    type Params = TodoParams;
    type Loader = TodoLoader;
    type Action = TodoAction;
    type View = TodoViewImpl;

    fn path() -> &'static str { "/" }
    fn loader(&self) -> Self::Loader { TodoLoader }
    fn action(&self) -> Self::Action { TodoAction }
    fn view(&self) -> Self::View { TodoViewImpl }
}

// [REQUIRED] 7. Define a Plate for explicit composition
pub struct TodoPlate;
#[async_trait::async_trait]
impl Plate<MyConfig> for TodoPlate {
    fn name(&self) -> &'static str { "todo" }
    fn dependencies(&self) -> Vec<&'static str> { vec![] }
    async fn init(&self, _ctx: &mut PlateContext<MyConfig>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    fn register_routes(&self, router: &mut Router<MyConfig>) {
        router.register(TodoRoute);
    }
}

// [REQUIRED] 8. UI Components
#[component]
fn TodoApp() -> impl IntoView {
    view! {
        <div class="p-8 max-w-md mx-auto bg-white rounded-xl shadow-md mt-10">
            <h1 class="text-2xl font-bold mb-4">"MontRS Todo"</h1>
            <p>"Scaffolded Explicit Architecture example."</p>
        </div>
    }
}

// [REQUIRED] 9. Main Entry Point (Explicit Bootstrapping)
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MyConfig { db_url: ":memory:".to_string() };
    let env = MyEnv;

    let spec = AppSpec::new(config, env)
        .with_target(Target::Server)
        .with_plate(Box::new(TodoPlate));

    println!("App ready with plates: {:?}", spec.plates.iter().map(|p| p.name()).collect::<Vec<_>>());

    // [EXPLICIT] Demonstrate Schema Validation
    let valid_todo = CreateTodo {
        title: "Build with Leptos".to_string(),
    };
    println!("Validation check: {:?}", valid_todo.validate());

    // [EXPLICIT] Mount or boot the application
    println!("Mounting Leptos application...");
    spec.mount(|| view! { <TodoApp /> });

    Ok(())
}
