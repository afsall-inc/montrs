use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError};
use montrs_ui::prelude::*;
use montrs_icons::*;

#[derive(Clone)]
struct MyEnv;
impl EnvConfig for MyEnv {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        match key {
            "APP_NAME" => Ok("MontRS".to_string()),
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
                    <span class="text-lg font-bold">"MontRS"</span>
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
                    "A modern Rust web framework built on Leptos. Type-safe, reactive, and explicit."
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
                    "Everything you need"
                </h2>
                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                    <FeatureCard
                        title="Type-Safe Architecture"
                        description="Build with confidence using MontRS's explicit architecture patterns. Plates, Routes, and Loaders are all fully typed."
                    />
                    <FeatureCard
                        title="Reactive by Default"
                        description="Powered by Leptos signals. Every component is reactive, efficient, and compile-time checked."
                    />
                    <FeatureCard
                        title="SQL-First ORM"
                        description="MontRS ORM provides a backend-agnostic, SQL-first database abstraction layer with type-safe query builders."
                    />
                    <FeatureCard
                        title="Built-in Validation"
                        description="Compile-time validation with proc macros. Define rules on your data types and get automatic checking."
                    />
                    <FeatureCard
                        title="Beautiful UI Components"
                        description="Shadcn-inspired UI components with Tailwind CSS. Dark mode, theming, and accessible components."
                    />
                    <FeatureCard
                        title="Developer Experience"
                        description="Hot reload, CLI tools, agents, and CI/CD integration. Built for productivity from day one."
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