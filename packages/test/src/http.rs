// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! In-process HTTP and route testing — no server, no socket.
//!
//! [`TestClient`] renders requests straight through the app's axum router via
//! [`montrs_core::serve::SsrApp`], and the harness can execute a route's loader
//! or action directly through [`montrs_core::RouteRef`].

use crate::kernel::TestHarness;
use montrs_core::{
    AppConfig, AppSpec, RouteContext, RouteError,
    serve::{SsrApp, SsrRequest},
};
use serde::{Serialize, de::DeserializeOwned};

/// A response from an in-process request.
#[derive(Debug, Clone)]
pub struct TestResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers (name, value).
    pub headers: Vec<(String, String)>,
    /// Raw response body.
    pub body: Vec<u8>,
}

impl TestResponse {
    /// True for 2xx statuses.
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// The body decoded as UTF-8 (lossy).
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    /// The rendered HTML with Leptos hot-reload markers removed — suitable for
    /// snapshot comparison.
    pub fn rendered_html(&self) -> String {
        strip_hot_reload_markers(&self.text())
    }

    /// Deserialize the body as JSON.
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    /// Look up a header value (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// Assert the status code.
    #[track_caller]
    pub fn assert_status(&self, expected: u16) -> &Self {
        assert_eq!(
            self.status,
            expected,
            "expected status {expected}, got {} (body: {})",
            self.status,
            self.text()
        );
        self
    }

    /// Assert a header contains a substring.
    #[track_caller]
    pub fn assert_header_contains(&self, name: &str, needle: &str) -> &Self {
        let value = self.header(name).unwrap_or("");
        assert!(
            value.contains(needle),
            "expected header {name:?} to contain {needle:?}, got {value:?}"
        );
        self
    }

    /// Assert the body text contains a substring.
    #[track_caller]
    pub fn assert_body_contains(&self, needle: &str) -> &Self {
        let body = self.text();
        assert!(
            body.contains(needle),
            "expected body to contain {needle:?}, got: {body}"
        );
        self
    }
}

/// Remove `<!--hot-reload|…|open-->` / `…|close-->` markers from HTML.
pub fn strip_hot_reload_markers(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("<!--hot-reload|") {
        out.push_str(&rest[..start]);
        match rest[start..].find("-->") {
            Some(end) => rest = &rest[start + end + 3..],
            None => {
                rest = "";
                break;
            }
        }
    }
    out.push_str(rest);
    out
}

/// A client that drives an app's router in-process.
pub struct TestClient {
    app: SsrApp,
}

impl TestClient {
    /// Build a client from an app spec and a root view factory.
    pub fn new<C, F, IV>(
        spec: &AppSpec<C>,
        root: F,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        C: AppConfig + 'static,
        F: Fn() -> IV + Clone + Send + Sync + 'static,
        IV: montrs_core::IntoView + 'static,
    {
        Ok(Self {
            app: SsrApp::build(spec.router.clone(), root)?,
        })
    }

    /// Send a fully specified request.
    pub fn request(
        &self,
        request: SsrRequest,
    ) -> Result<TestResponse, Box<dyn std::error::Error>> {
        let (status, headers, body) = self.app.render_request(request)?;
        Ok(TestResponse {
            status,
            headers,
            body,
        })
    }

    /// Send a `GET` request.
    pub fn get(
        &self,
        uri: &str,
    ) -> Result<TestResponse, Box<dyn std::error::Error>> {
        self.request(SsrRequest::get(uri))
    }

    /// Send a `POST` with a JSON body.
    pub fn post_json<T: Serialize>(
        &self,
        uri: &str,
        value: &T,
    ) -> Result<TestResponse, Box<dyn std::error::Error>> {
        self.request(SsrRequest::new("POST", uri).with_json(value)?)
    }
}

#[cfg(feature = "http")]
impl<C: AppConfig> TestHarness<C> {
    /// Resolve a route path for inspection and in-process execution.
    pub fn route(&self, path: &str) -> Option<montrs_core::RouteRef<'_, C>> {
        self.spec.router.route(path)
    }

    /// The route context used for loader/action execution.
    pub fn route_context(&self) -> RouteContext<'_, C> {
        RouteContext {
            config: &self.spec.config,
            env: &self.spec.env,
        }
    }

    /// Execute the loader for `path` in-process.
    pub async fn load(
        &self,
        path: &str,
    ) -> Result<serde_json::Value, RouteError> {
        let route = self.route(path).ok_or(RouteError::NotFound)?;
        route.load(self.route_context()).await
    }

    /// Execute the action for `path` in-process with `input`.
    pub async fn act(
        &self,
        path: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, RouteError> {
        let route = self.route(path).ok_or(RouteError::NotFound)?;
        route.act(self.route_context(), input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;
    use montrs_core::{
        AppConfig, AppSpec, EnvConfig, NoParams, NoopAction, Route,
        RouteLoader, RouteView, env::EnvError,
    };

    #[derive(Clone)]
    struct Env;
    impl EnvConfig for Env {
        fn get_var(&self, key: &str) -> Result<String, EnvError> {
            match key {
                "GREETING" => Ok("hello".to_string()),
                _ => Err(EnvError::MissingKey(key.to_string())),
            }
        }
    }

    #[derive(Clone)]
    struct Cfg;
    impl AppConfig for Cfg {
        type Error = std::io::Error;
        type Env = Env;
    }

    struct EchoView;
    impl RouteView for EchoView {
        fn render(&self) -> impl IntoView {
            view! { <p>"echo"</p> }
        }
    }

    struct EchoLoader;
    #[async_trait::async_trait]
    impl RouteLoader<NoParams, Cfg> for EchoLoader {
        type Output = String;
        async fn load(
            &self,
            ctx: RouteContext<'_, Cfg>,
            _params: NoParams,
        ) -> Result<String, RouteError> {
            Ok(ctx.env.get_var("GREETING").unwrap_or_default())
        }
    }

    struct EchoRoute;
    impl Route<Cfg> for EchoRoute {
        type Params = NoParams;
        type Loader = EchoLoader;
        type Action = NoopAction;
        type View = EchoView;
        fn path() -> &'static str {
            "/echo"
        }
        fn loader(&self) -> EchoLoader {
            EchoLoader
        }
        fn action(&self) -> NoopAction {
            NoopAction
        }
        fn view(&self) -> EchoView {
            EchoView
        }
    }

    fn spec() -> AppSpec<Cfg> {
        let mut spec = AppSpec::new(Cfg, Env);
        spec.router.register(EchoRoute);
        spec
    }

    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    #[test]
    fn resolves_route_and_loads_through_env() {
        let harness = TestHarness::new(spec());
        let route = harness.route("/echo").expect("route resolves");
        assert_eq!(route.path(), "/echo");

        let loaded = block_on(harness.load("/echo")).unwrap();
        assert_eq!(loaded, serde_json::Value::String("hello".to_string()));
    }

    #[test]
    fn missing_route_is_not_found() {
        let harness = TestHarness::new(spec());
        let err = block_on(harness.load("/nope")).unwrap_err();
        assert!(matches!(err, RouteError::NotFound));
    }

    #[test]
    fn client_renders_a_get_in_process() {
        let client =
            TestClient::new(&spec(), || view! { <p>"root"</p> }).unwrap();
        let res = client.get("/echo").unwrap();
        res.assert_status(200);
        res.assert_body_contains("root");
        // Hot-reload markers are stripped for stable snapshots.
        assert!(res.rendered_html().contains("<p>root</p>"));
    }

    #[test]
    fn strip_markers_is_idempotent_on_clean_html() {
        let html = "<!--hot-reload|a|open--><p>x</p><!--hot-reload|a|close-->";
        assert_eq!(strip_hot_reload_markers(html), "<p>x</p>");
        assert_eq!(strip_hot_reload_markers("<p>y</p>"), "<p>y</p>");
    }
}
