use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError};
use montrs_ui::prelude::*;
use montrs_icons::*;

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

#[derive(Clone)]
struct MyAppConfig;
impl AppConfig for MyAppConfig {
    type Error = crate::MyAppError;
    type Env = MyEnv;
}

#[derive(Debug, thiserror::Error)]
pub enum MyAppError {
    #[error("Internal: {0}")]
    Internal(String),
}

#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <div class="min-h-screen bg-background text-foreground">
                <Header />
                <Hero />
                <Features />
                <Footer />
            </div>
        </ThemeProvider>
    }
}

#[component]
fn Header() -> impl IntoView {
    let theme = use_theme();

    view! {
        <header class="border-b border-border">
            <div class="mx-auto flex h-16 max-w-6xl items-center justify-between px-6">
                <div class="flex items-center gap-2">
                    <RocketIcon class="h-6 w-6 text-primary" />
                    <span class="text-lg font-bold">"MontRS Workspace"</span>
                </div>
                <nav class="flex items-center gap-4">
                    <a
                        href="https://opencode.ai"
                        class="text-sm text-muted-foreground transition-colors hover:text-foreground"
                    >
                        "Docs"
                    </a>
                    <button
                        on:click=move |_| toggle_theme()
                        class="rounded-md p-2 transition-colors hover:bg-accent"
                        aria-label="Toggle theme"
                    >
                        {move || {
                            if theme.get().is_dark() {
                                view! { <SunIcon class="h-5 w-5" /> }.into_any()
                            } else {
                                view! { <MoonIcon class="h-5 w-5" /> }.into_any()
                            }
                        }}
                    </button>
                </nav>
            </div>
        </header>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="py-24 text-center">
            <div class="mx-auto max-w-4xl px-6">
                <h1 class="text-5xl font-bold tracking-tight sm:text-6xl">
                    "Welcome to MontRS"
                </h1>
                <p class="mx-auto mt-6 max-w-2xl text-lg text-muted-foreground">
                    "A multi-package workspace with shared UI components."
                </p>
                <div class="mt-10 flex items-center justify-center gap-4">
                    <a
                        href="/docs"
                        class="inline-flex items-center gap-2 rounded-md bg-primary px-6 py-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
                    >
                        "Get Started"
                        <ArrowRightIcon class="h-4 w-4" />
                    </a>
                    <a
                        href="https://github.com/anomalyco/montrs"
                        class="inline-flex items-center gap-2 rounded-md border border-border px-6 py-3 text-sm font-medium transition-colors hover:bg-accent"
                    >
                        <GitBranchIcon class="h-4 w-4" />
                        "View on GitHub"
                    </a>
                </div>
                <p class="mt-8 text-sm text-muted-foreground">
                    "Using shared UI from " <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-xs">"packages/ui"</code>
                </p>
            </div>
        </section>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="mx-auto max-w-6xl px-6">
                <h2 class="mb-12 text-center text-3xl font-bold">
                    "Workspace features"
                </h2>
                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                    <FeatureCard
                        title="Shared Components"
                        description="Reusable UI components defined in packages/ui, consumed by all apps in the workspace."
                    />
                    <FeatureCard
                        title="Unified Config"
                        description="Workspace-level Cargo.toml manages shared dependencies, versions, and settings."
                    />
                    <FeatureCard
                        title="Consistent Theming"
                        description="Dark/light mode, Tailwind CSS variables, and design tokens shared across all apps."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-6 transition-shadow hover:shadow-lg">
            <h3 class="mb-2 font-semibold">{title}</h3>
            <p class="text-sm text-muted-foreground">{description}</p>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-border py-8">
            <div class="mx-auto max-w-6xl px-6 text-center text-sm text-muted-foreground">
                <p>
                    "Built with " <strong>"MontRS"</strong> " & " <strong>"Leptos"</strong>
                </p>
            </div>
        </footer>
    }
}

fn main() {
    let _spec = AppSpec::new(MyAppConfig, MyEnv).with_target(Target::Wasm);
    mount_to_body(|| view! { <App /> });
}