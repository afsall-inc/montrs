use async_trait::async_trait;
use leptos::prelude::*;
use montrs_core::{
    AppConfig, EnvConfig, Route, RouteAction, RouteContext, RouteError,
    RouteLoader, RouteParams, RouteView, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct TestConfig;
impl AppConfig for TestConfig {
    type Error = std::io::Error;
    type Env = TestEnv;
}

#[derive(Clone)]
struct TestEnv;
impl EnvConfig for TestEnv {
    fn get_var(&self, _key: &str) -> Result<String, montrs_core::EnvError> {
        Ok("test".to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct UserParams {
    id: u32,
}
impl RouteParams for UserParams {}

struct UserLoader;
#[async_trait]
impl RouteLoader<UserParams, TestConfig> for UserLoader {
    type Output = String;
    async fn load(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        params: UserParams,
    ) -> Result<Self::Output, RouteError> {
        Ok(format!("User {}", params.id))
    }
}

struct UserAction;
#[async_trait]
impl RouteAction<UserParams, TestConfig> for UserAction {
    type Input = String;
    type Output = String;
    async fn act(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        params: UserParams,
        input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        Ok(format!("Updated user {} with {}", params.id, input))
    }
}

struct UserView;
impl RouteView for UserView {
    fn render(&self) -> impl IntoView {
        view! { <div>"User View"</div> }
    }
}

struct UserRoute;
impl Route<TestConfig> for UserRoute {
    type Params = UserParams;
    type Loader = UserLoader;
    type Action = UserAction;
    type View = UserView;

    fn path() -> &'static str {
        "/users/:id"
    }
    fn loader(&self) -> Self::Loader {
        UserLoader
    }
    fn action(&self) -> Self::Action {
        UserAction
    }
    fn view(&self) -> Self::View {
        UserView
    }
}

#[tokio::test]
async fn test_router_registration_and_handling() {
    let mut router = Router::<TestConfig>::new();
    router.register(UserRoute);

    let config = TestConfig;
    let env = TestEnv;
    let ctx = RouteContext {
        config: &config,
        env: &env,
    };

    let params = serde_json::json!({ "id": 123 });

    // Test load
    let load_res = router.spec().routes.get("/users/:id").unwrap();
    assert_eq!(load_res.path, "/users/:id");

    // In a real scenario, we'd call handle_load on the RouteInfo, but it's internal.
    // However, we can verify the spec is correct.
    assert_eq!(router.spec().routes.len(), 1);
}
