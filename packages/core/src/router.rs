// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
// Path segment types for pattern matching
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    Static(String),
    Param(String),
    Splat(String),
}

fn parse_pattern(pattern: &str) -> Vec<Segment> {
    pattern
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|seg| {
            if let Some(name) = seg.strip_prefix(':') {
                Segment::Param(name.to_string())
            } else if let Some(name) = seg.strip_prefix('*') {
                Segment::Splat(name.to_string())
            } else {
                Segment::Static(seg.to_string())
            }
        })
        .collect()
}

fn match_path(
    pattern: &[Segment],
    url_path: &str,
) -> Option<HashMap<String, String>> {
    let url_segs: Vec<&str> =
        url_path.split('/').filter(|s| !s.is_empty()).collect();

    let mut params = HashMap::new();
    let mut pi = 0;
    let mut ui = 0;

    while pi < pattern.len() {
        match &pattern[pi] {
            Segment::Static(s) => {
                if ui >= url_segs.len() || url_segs[ui] != s {
                    return None;
                }
                ui += 1;
                pi += 1;
            }
            Segment::Param(name) => {
                if ui >= url_segs.len() {
                    return None;
                }
                params.insert(name.clone(), url_segs[ui].to_string());
                ui += 1;
                pi += 1;
            }
            Segment::Splat(name) => {
                let rest: Vec<&str> = url_segs[ui..].to_vec();
                params.insert(name.clone(), rest.join("/"));
                return Some(params);
            }
        }
    }

    if ui != url_segs.len() {
        return None;
    }

    Some(params)
}

// ---------------------------------------------------------------------------
// The Application Router
// ---------------------------------------------------------------------------

type RouteEntry<C> = (String, Vec<Segment>, Arc<dyn RouteInfo<C>>);

/// The Application Router which maintains the static route graph.
#[derive(Clone)]
pub struct Router<C: AppConfig> {
    routes: Vec<RouteEntry<C>>,
    /// Fast lookup for exact-match static routes (no params).
    exact_routes: HashMap<&'static str, Arc<dyn RouteInfo<C>>>,
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
            routes: Vec::new(),
            exact_routes: HashMap::new(),
        }
    }

    pub fn register<R: Route<C>>(&mut self, route: R) {
        let path = R::path();
        let segments = parse_pattern(path);
        let arc: Arc<dyn RouteInfo<C>> = Arc::new(route);

        // Static routes go into fast lookup
        let has_params = segments
            .iter()
            .any(|s| matches!(s, Segment::Param(_) | Segment::Splat(_)));
        if !has_params {
            self.exact_routes.insert(path, arc.clone());
        }

        self.routes.push((path.to_string(), segments, arc));
    }

    /// Resolves a path to a `RouteView` and returns its view.
    /// Supports static routes, `:param` patterns, `*splat` catch-all, and built-in 404.
    pub fn render_view(&self, path: &str) -> AnyView {
        // 1. Try fast exact match
        if let Some(route) = self.exact_routes.get(path) {
            return (route.render())();
        }

        // 2. Try pattern matching (params, optional params, splats)
        for (_, segments, route) in &self.routes {
            if match_path(segments, path).is_some() {
                return (route.render())();
            }
        }

        // 3. Try catch-all
        if let Some(catch_all) = self.exact_routes.get("*") {
            return (catch_all.render())();
        }
        for (path_str, _, route) in &self.routes {
            if path_str == "*" {
                return (route.render())();
            }
        }

        // 4. Built-in 404
        view! {
            <div class="flex flex-col items-center justify-center min-h-[60vh]">
                <h1 class="text-4xl font-bold">
                    "404"
                </h1>
                <p class="text-muted-foreground">
                    "Page not found"
                </p>
            </div>
        }
        .into_any()
    }

    /// Convert MontRS routes to Axum route listings for SSR integration.
    /// Each registered route maps to an AxumRouteListing with SsrMode::OutOfOrder.
    #[cfg(feature = "ssr")]
    pub fn to_axum_route_listings(&self) -> Vec<leptos_axum::AxumRouteListing> {
        use leptos_router::SsrMode;
        self.routes
            .iter()
            .map(|(path, _, _)| {
                leptos_axum::AxumRouteListing::new(
                    path.clone(),
                    SsrMode::OutOfOrder,
                    [leptos_router::Method::Get],
                    vec![],
                )
            })
            .collect()
    }

    /// Resolve `path` to a [`RouteRef`] for inspection, loader/action execution,
    /// and rendering — without going through HTTP.
    ///
    /// Returns `None` when no route (not even a `*` catch-all) matches.
    pub fn route(&self, path: &str) -> Option<RouteRef<'_, C>> {
        if let Some(route) = self.exact_routes.get(path) {
            return Some(RouteRef {
                inner: route,
                params: serde_json::Value::Object(Default::default()),
            });
        }

        for (_, segments, route) in &self.routes {
            if let Some(params) = match_path(segments, path) {
                return Some(RouteRef {
                    inner: route,
                    params: params_to_json(params),
                });
            }
        }

        if let Some(catch_all) = self.exact_routes.get("*") {
            return Some(RouteRef {
                inner: catch_all,
                params: serde_json::Value::Object(Default::default()),
            });
        }
        for (path_str, _, route) in &self.routes {
            if path_str == "*" {
                return Some(RouteRef {
                    inner: route,
                    params: serde_json::Value::Object(Default::default()),
                });
            }
        }

        None
    }

    pub fn spec(&self) -> RouterSpec {
        let mut routes = HashMap::new();
        for (path, _, route) in &self.routes {
            routes.insert(path.clone(), route.metadata());
        }
        RouterSpec { routes }
    }
}

/// A resolved route, for inspection and in-process execution.
///
/// Obtained from [`Router::route`]. Lets tests (and tooling) run a route's
/// loader or action and render its view without a running server.
pub struct RouteRef<'a, C: AppConfig> {
    inner: &'a Arc<dyn RouteInfo<C>>,
    params: serde_json::Value,
}

impl<C: AppConfig> RouteRef<'_, C> {
    /// The route's path pattern (e.g. `/users/:id`).
    pub fn path(&self) -> &'static str {
        self.inner.path()
    }

    /// The extracted path parameters as JSON.
    pub fn params(&self) -> &serde_json::Value {
        &self.params
    }

    /// Render the route's view.
    pub fn render(&self) -> AnyView {
        (self.inner.render())()
    }

    /// Route metadata (loader/action descriptions).
    pub fn metadata(&self) -> RouteMetadata {
        self.inner.metadata()
    }

    /// Execute the route's loader in-process.
    pub async fn load(
        &self,
        ctx: RouteContext<'_, C>,
    ) -> Result<serde_json::Value, RouteError> {
        self.inner.handle_load(ctx, self.params.clone()).await
    }

    /// Execute the route's action in-process.
    pub async fn act(
        &self,
        ctx: RouteContext<'_, C>,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, RouteError> {
        self.inner.handle_act(ctx, self.params.clone(), input).await
    }
}

fn params_to_json(params: HashMap<String, String>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in params {
        map.insert(k, serde_json::Value::String(v));
    }
    serde_json::Value::Object(map)
}

// ---------------------------------------------------------------------------
// Reactive client-side components
// ---------------------------------------------------------------------------
/// Reads the MontRS Router from Leptos context.
pub fn use_montrs_router<C: AppConfig + 'static>() -> Router<C> {
    use_context::<Router<C>>().expect(
        "MontRS Router not found in context. Did you forget to call \
         AppSpec::mount_with_router?",
    )
}

/// Document-level click guard that keeps navigation sane under
/// [`RouterOutlet`].
///
/// Leptos Router's anchor interception defers `history.pushState` until its
/// `<Routes>` tree resolves — which never happens when the app renders through
/// [`RouterOutlet`]. This guard:
///
/// * neutralises placeholder `href="#"` anchors (the router would otherwise
///   treat them as a navigation to `/`), and
/// * drives internal `href="/..."` anchors through `use_navigate`, so the URL
///   bar updates immediately.
///
/// It is registered in the capture phase and deliberately does **not** stop
/// propagation, so component-level `on:click` handlers still run. It is
/// auto-installed by [`RouterOutlet`]; apps never need to add it themselves.
#[allow(non_snake_case)]
#[component]
pub fn RouterAnchorGuard() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let navigate = leptos_router::hooks::use_navigate();

        Effect::new(move |_| {
            use leptos::wasm_bindgen::{JsCast, prelude::Closure};

            let navigate = navigate.clone();
            let Some(window) = leptos::web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };

            let cb = Closure::<dyn FnMut(leptos::web_sys::MouseEvent)>::wrap(
                Box::new(move |ev: leptos::web_sys::MouseEvent| {
                    if ev.default_prevented() {
                        return;
                    }
                    let Some(target) = ev.target().and_then(|t| {
                        t.dyn_into::<leptos::web_sys::Element>().ok()
                    }) else {
                        return;
                    };
                    let Ok(Some(anchor)) = target.closest("a") else {
                        return;
                    };
                    let href = anchor.get_attribute("href").unwrap_or_default();

                    // Only bare `#` placeholders are neutralised; real
                    // fragment links (e.g. the skip link) must keep working.
                    if href.is_empty() || href == "#" {
                        ev.prevent_default();
                        return;
                    }
                    if !href.starts_with('/') || href.starts_with("//") {
                        return;
                    }
                    if ev.meta_key()
                        || ev.ctrl_key()
                        || ev.shift_key()
                        || ev.alt_key()
                    {
                        return;
                    }
                    if anchor.get_attribute("target").is_some()
                        || anchor.get_attribute("download").is_some()
                    {
                        return;
                    }

                    ev.prevent_default();
                    navigate(&href, Default::default());
                }),
            );

            let _ = document.add_event_listener_with_callback_and_bool(
                "click",
                cb.as_ref().unchecked_ref(),
                true,
            );
            cb.forget();
        });
    }

    view! {
        <span class="hidden" aria-hidden="true" />
    }
}

/// Renders the matched route's view. Place inside your layout.
///
/// Watches the current URL path via Leptos Router's `use_location` and
/// renders the corresponding `RouteView` from the MontRS `Router<C>`. It also
/// installs [`RouterAnchorGuard`] so plain `<a href="/...">` links navigate
/// correctly under the custom outlet.
#[allow(non_snake_case)]
pub fn RouterOutlet<C: AppConfig + 'static>() -> impl IntoView {
    let router = use_montrs_router::<C>();
    let location = leptos_router::hooks::use_location();

    view! {
        <RouterAnchorGuard />
        {move || {
            let path = location.pathname.get();
            router.render_view(&path)
        }}
    }
}

/// A client-side navigation link.
///
/// Navigates with `use_navigate` on a plain `<a>` so the URL bar updates
/// immediately. Leptos Router's `<A>`/anchor interception defers
/// `history.pushState` until its `<Routes>` tree resolves, which never
/// happens when the app renders through [`RouterOutlet`] — so `<A>` cannot
/// be used here. The link is automatically marked active for the current
/// route. Users never import Leptos Router directly.
#[allow(non_snake_case)]
pub fn RouteLink<C: AppConfig + 'static>(
    to: &'static str,
    children: ChildrenFn,
    class: Option<Signal<String>>,
) -> impl IntoView {
    let class_val = class.unwrap_or_else(|| Signal::from(String::new()));
    let _router = use_montrs_router::<C>();
    let navigate = leptos_router::hooks::use_navigate();
    let to_owned = to.to_string();

    // Active detection mirrors `<A>`'s default: exact match or nested under
    // `to/`. Exposed as a `Signal` so both the class and `aria-current` can
    // read it reactively as the route changes.
    let location = leptos_router::hooks::use_location();
    let active = Signal::derive(move || {
        let current = location.pathname.get();
        current == to || (to != "/" && current.starts_with(&format!("{to}/")))
    });

    let a_class = move || {
        let base = class_val.get();
        if active.get() {
            format!("{} active", base)
        } else {
            base
        }
    };

    view! {
        <a href=to_owned class=a_class data-montrs-route=to aria-current=move || active.get().then_some("page") on:click=move |ev| {
                ev.prevent_default();
                navigate(to, Default::default());
            }>
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
