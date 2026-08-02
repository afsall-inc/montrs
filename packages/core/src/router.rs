//! montrs-core/src/router.rs: Explicit routing primitives inspired by Remix.
//!
//! This file defines the core traits and structs for the MontRS Router,
//! ensuring deterministic data loading, mutation, and navigation across platforms.

use crate::AppConfig;
use async_trait::async_trait;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

/// Trait for route parameters. Must be serializable and deserializable.
pub trait RouteParams:
    Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static
{
}

/// Trait for data loading components. Loaders are responsible for fetching data
/// for a specific route. They are read-only and idempotent.
#[async_trait]
pub trait RouteLoader<P: RouteParams, C: AppConfig>:
    Send + Sync + 'static
{
    type Output: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    async fn load(
        &self,
        ctx: RouteContext<'_, C>,
        params: P,
    ) -> Result<Self::Output, RouteError>;

    /// Returns a description of what this loader fetches.
    fn description(&self) -> &'static str {
        ""
    }
}

/// Trait for data mutation components. Actions are responsible for handling
/// state-changing operations (form submissions, API mutations).
#[async_trait]
pub trait RouteAction<P: RouteParams, C: AppConfig>:
    Send + Sync + 'static
{
    type Input: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;
    type Output: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    async fn act(
        &self,
        ctx: RouteContext<'_, C>,
        params: P,
        input: Self::Input,
    ) -> Result<Self::Output, RouteError>;

    /// Returns a description of what this action does.
    fn description(&self) -> &'static str {
        ""
    }
}

/// Trait for the visual representation of a route.
pub trait RouteView: Send + Sync + 'static {
    fn render(&self) -> impl IntoView;
}

/// The core Route trait that unifies params, loader, action, and view.
pub trait Route<C: AppConfig>: Send + Sync + 'static {
    type Params: RouteParams;
    type Loader: RouteLoader<Self::Params, C>;
    type Action: RouteAction<Self::Params, C>;
    type View: RouteView;

    /// The path pattern for this route (e.g., "/users/:id").
    fn path() -> &'static str;

    /// Returns the loader instance for this route.
    fn loader(&self) -> Self::Loader;

    /// Returns the action instance for this route.
    fn action(&self) -> Self::Action;

    /// Returns the view instance for this route.
    fn view(&self) -> Self::View;
}

/// Context passed to loaders and actions, providing access to the application configuration and state.
pub struct RouteContext<'a, C: AppConfig> {
    pub config: &'a C,
    pub env: &'a dyn crate::env::EnvConfig,
}

/// Standard error type for router operations.
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum RouteError {
    #[error("Route not found")]
    NotFound,
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("Internal router error: {0}")]
    InternalError(String),
    #[error("External error: {0}")]
    External(String),
}

/// Standard response format for a Loader (for serialization).
#[derive(Serialize, Deserialize)]
pub struct LoaderResponse {
    pub data: serde_json::Value,
}

/// Standard response format for an Action (for serialization).
#[derive(Serialize, Deserialize)]
pub struct ActionResponse {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Convenience types for view-only routes (no params, no loader, no action)
// ---------------------------------------------------------------------------

/// Empty params for routes that don't extract path parameters.
#[derive(Serialize, Deserialize)]
pub struct NoParams;
impl RouteParams for NoParams {}

/// A no-op loader that returns `()`.
pub struct NoopLoader;

#[async_trait]
impl<P: RouteParams, C: AppConfig> RouteLoader<P, C> for NoopLoader {
    type Output = ();
    async fn load(
        &self,
        _ctx: RouteContext<'_, C>,
        _params: P,
    ) -> Result<Self::Output, RouteError> {
        Ok(())
    }
}

/// A no-op action that does nothing.
pub struct NoopAction;

#[async_trait]
impl<P: RouteParams, C: AppConfig> RouteAction<P, C> for NoopAction {
    type Input = ();
    type Output = ();
    async fn act(
        &self,
        _ctx: RouteContext<'_, C>,
        _params: P,
        _input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The Application Router
// ---------------------------------------------------------------------------

/// The Application Router which maintains the static route graph.
#[derive(Clone)]
pub struct Router<C: AppConfig> {
    routes: HashMap<&'static str, Arc<dyn RouteInfo<C>>>,
}

/// Internal trait to erase the associated types of a Route for storage in the Router.
#[async_trait]
#[allow(dead_code)]
trait RouteInfo<C: AppConfig>: Send + Sync + 'static {
    fn path(&self) -> &'static str;
    async fn handle_load(
        &self,
        ctx: RouteContext<'_, C>,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, RouteError>;
    async fn handle_act(
        &self,
        ctx: RouteContext<'_, C>,
        params: serde_json::Value,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, RouteError>;
    fn render(&self) -> Box<dyn Fn() -> AnyView + Send + Sync>;
    fn metadata(&self) -> RouteMetadata;
}

#[async_trait]
impl<C: AppConfig, R: Route<C>> RouteInfo<C> for R {
    fn path(&self) -> &'static str {
        R::path()
    }

    async fn handle_load(
        &self,
        ctx: RouteContext<'_, C>,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, RouteError> {
        let params: R::Params = serde_json::from_value(params)
            .map_err(|e| RouteError::ValidationFailed(e.to_string()))?;

        let loader = self.loader();
        let output = loader.load(ctx, params).await?;
        serde_json::to_value(output)
            .map_err(|e| RouteError::InternalError(e.to_string()))
    }

    async fn handle_act(
        &self,
        ctx: RouteContext<'_, C>,
        params: serde_json::Value,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, RouteError> {
        let params: R::Params = serde_json::from_value(params)
            .map_err(|e| RouteError::ValidationFailed(e.to_string()))?;
        let input: <R::Action as RouteAction<R::Params, C>>::Input =
            serde_json::from_value(input)
                .map_err(|e| RouteError::ValidationFailed(e.to_string()))?;

        let action = self.action();
        let output = action.act(ctx, params, input).await?;
        serde_json::to_value(output)
            .map_err(|e| RouteError::InternalError(e.to_string()))
    }

    fn render(&self) -> Box<dyn Fn() -> AnyView + Send + Sync> {
        let view = self.view();
        Box::new(move || view.render().into_any())
    }

    fn metadata(&self) -> RouteMetadata {
        RouteMetadata {
            path: R::path().to_string(),
            loader_description: self.loader().description().to_string(),
            action_description: self.action().description().to_string(),
        }
    }
}

impl<C: AppConfig> Default for Router<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AppConfig> Router<C> {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register<R: Route<C>>(&mut self, route: R) {
        self.routes.insert(R::path(), Arc::new(route));
    }

    /// Resolves a path to a `RouteView` renderer and returns its view.
    /// Falls back to a catch-all route registered at `"*"`, or a built-in 404.
    pub fn render_view(&self, path: &str) -> AnyView {
        // Try exact match first
        if let Some(route) = self.routes.get(path) {
            return (route.render())();
        }
        // Try catch-all
        if let Some(catch_all) = self.routes.get("*") {
            return (catch_all.render())();
        }
        // Built-in 404
        (|| {
            view! {
                <div class="flex flex-col items-center justify-center min-h-[60vh]">
                    <h1 class="text-4xl font-bold">"404"</h1>
                    <p class="text-muted-foreground">"Page not found"</p>
                </div>
            }
            .into_any()
        })()
    }

    pub fn spec(&self) -> RouterSpec {
        let mut routes = HashMap::new();
        for (path, route) in &self.routes {
            routes.insert(path.to_string(), route.metadata());
        }
        RouterSpec { routes }
    }
}

// ---------------------------------------------------------------------------
// Reactive client-side components (manual — no #[component] macro)
// ---------------------------------------------------------------------------

/// Reads the MontRS Router from Leptos context.
pub fn use_montrs_router<C: AppConfig + 'static>() -> Router<C> {
    use_context::<Router<C>>().expect("MontRS Router not found in context. Did you forget to call AppSpec::mount_with_router?")
}

/// Renders the matched route's view. Place inside your layout.
///
/// Watches the current URL path via Leptos Router's `use_location` and
/// renders the corresponding `RouteView` from the MontRS `Router<C>`.
#[allow(non_snake_case)]
pub fn RouterOutlet<C: AppConfig + 'static>() -> impl IntoView {
    let router = use_montrs_router::<C>();
    let location = leptos_router::hooks::use_location();

    let view = move || {
        let path = location.pathname.get();
        router.render_view(&path)
    };

    view
}

/// A client-side navigation link.
///
/// Wraps Leptos Router's `<A>` component internally. Users never import
/// Leptos Router directly.
#[allow(non_snake_case)]
pub fn RouteLink<C: AppConfig + 'static>(
    to: &'static str,
    children: ChildrenFn,
    class: Option<Signal<String>>,
) -> impl IntoView {
    let class_val = class.unwrap_or_else(|| Signal::from(String::new()));
    let _router = use_montrs_router::<C>();

    let is_active = {
        let to = to.to_string();
        let location = leptos_router::hooks::use_location();
        move || {
            let current = location.pathname.get();
            let is_exact = current == to;
            let is_prefix = current.starts_with(&format!("{}/", to));
            is_exact || is_prefix
        }
    };

    let a_class = move || {
        let base = class_val.get();
        if is_active() {
            format!("{} active", base)
        } else {
            base
        }
    };

    view! {
        <a href=to class=a_class data-montrs-route=to>
            {children()}
        </a>
    }
}

// ---------------------------------------------------------------------------
// view_route! macro
// ---------------------------------------------------------------------------

/// Creates a view-only route struct with minimal boilerplate.
///
/// # Example
/// ```rust,ignore
/// use montrs_core::*;
/// 
/// struct HomeView;
/// impl RouteView for HomeView {
///     fn render(&self) -> impl IntoView {
///         view! { <h1>"Home"</h1> }
///     }
/// }
///
/// view_route! { HomeRoute, "/", HomeView }
/// ```
///
/// This expands to:
/// ```rust,ignore
/// pub struct HomeRoute;
/// impl<C: AppConfig> Route<C> for HomeRoute {
///     type Params = NoParams;
///     type Loader = NoopLoader;
///     type Action = NoopAction;
///     type View = HomeView;
///     fn path() -> &'static str { "/" }
///     fn loader(&self) -> Self::Loader { NoopLoader }
///     fn action(&self) -> Self::Action { NoopAction }
///     fn view(&self) -> Self::View { HomeView }
/// }
/// ```
#[macro_export]
macro_rules! view_route {
    ($name:ident, $path:expr, $view:path) => {
        pub struct $name;
        impl<C: $crate::AppConfig> $crate::Route<C> for $name {
            type Params = $crate::NoParams;
            type Loader = $crate::NoopLoader;
            type Action = $crate::NoopAction;
            type View = $view;

            fn path() -> &'static str {
                $path
            }

            fn loader(&self) -> Self::Loader {
                $crate::NoopLoader
            }

            fn action(&self) -> Self::Action {
                $crate::NoopAction
            }

            fn view(&self) -> Self::View {
                $view
            }
        }
    };
}

// ---------------------------------------------------------------------------
// RouterSpec for agent metadata
// ---------------------------------------------------------------------------

/// A machine-readable specification of the router.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterSpec {
    pub routes: HashMap<String, RouteMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteMetadata {
    pub path: String,
    pub loader_description: String,
    pub action_description: String,
}