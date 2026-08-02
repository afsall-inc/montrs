use leptos::prelude::*;
use montrs_core::{
    AppConfig, AppSpec, Plate, PlateContext, Route, RouteAction, RouteContext, RouteError,
    RouteLoader, RouteParams, RouteView, Router, Target,
};
use montrs_orm::{DbBackend, FromRow, SqliteBackend};
use montrs_validator::Validator;
use montrs_ui::prelude::*;
use montrs_icons::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum MyError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Generic error: {0}")]
    Generic(String),
}

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

#[derive(Clone)]
pub struct MyConfig {
    pub db_url: String,
}
impl AppConfig for MyConfig {
    type Error = MyError;
    type Env = MyEnv;
}

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

#[component]
fn TodoApp() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <ThemeProvider>
            <div class="min-h-screen bg-background text-foreground">
                <header class="border-b border-border">
                    <div class="mx-auto flex h-16 max-w-2xl items-center gap-2 px-6">
                        <CheckCheckIcon class="h-6 w-6 text-primary" />
                        <span class="text-lg font-bold">"MontRS Todo"</span>
                    </div>
                </header>
                <main class="mx-auto max-w-2xl px-6 py-12">
                    <div class="rounded-lg border border-border bg-card p-8">
                        <div class="flex items-center gap-3">
                            <ListChecksIcon class="h-8 w-8 text-primary" />
                            <div>
                                <h1 class="text-2xl font-bold">"Todo Manager"</h1>
                                <p class="text-sm text-muted-foreground">
                                    "Scaffolded Explicit Architecture example."
                                </p>
                            </div>
                        </div>
                        <div class="mt-8 flex items-center gap-4">
                            <button
                                on:click=move |_| set_count.update(|n| *n += 1)
                                class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
                            >
                                <PlusIcon class="h-4 w-4" />
                                "Count: " {count}
                            </button>
                            <span class="text-sm text-muted-foreground">
                                "Click to increment"
                            </span>
                        </div>
                        <div class="mt-6 rounded-md bg-muted p-4">
                            <p class="text-xs text-muted-foreground">
                                "This example demonstrates: AppSpec, Plate, Route, Loader, Action, Validator, ORM, and montrs-ui components."
                            </p>
                        </div>
                    </div>
                </main>
            </div>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MyConfig { db_url: ":memory:".to_string() };
    let env = MyEnv;

    let spec = AppSpec::new(config, env)
        .with_target(Target::Server)
        .with_plate(Box::new(TodoPlate));

    println!("App ready with plates: {:?}", spec.plates.iter().map(|p| p.name()).collect::<Vec<_>>());

    let valid_todo = CreateTodo {
        title: "Build with MontRS".to_string(),
    };
    println!("Validation check: {:?}", valid_todo.validate());

    println!("Mounting Leptos application...");
    spec.mount(|| view! { <TodoApp /> });

    Ok(())
}