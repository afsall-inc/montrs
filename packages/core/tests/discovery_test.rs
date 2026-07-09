use async_trait::async_trait;
use montrs_core::{
    AppConfig, RouteAction, RouteContext, RouteError, RouteLoader, RouteParams,
    LoaderResponse, ActionResponse,
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
impl montrs_core::EnvConfig for TestEnv {
    fn get_var(&self, _key: &str) -> Result<String, montrs_core::EnvError> {
        Ok("test".to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct EmptyParams;
impl RouteParams for EmptyParams {}

struct TestLoader;

#[async_trait]
impl RouteLoader<EmptyParams, TestConfig> for TestLoader {
    type Output = LoaderResponse;

    async fn load(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        _params: EmptyParams,
    ) -> Result<Self::Output, RouteError> {
        Ok(LoaderResponse {
            data: serde_json::json!({}),
        })
    }

    fn description(&self) -> &'static str {
        "A test loader for discovery verification"
    }
}

struct TestAction;

#[async_trait]
impl RouteAction<EmptyParams, TestConfig> for TestAction {
    type Input = serde_json::Value;
    type Output = ActionResponse;

    async fn act(
        &self,
        _ctx: RouteContext<'_, TestConfig>,
        _params: EmptyParams,
        _input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        Ok(ActionResponse {
            data: serde_json::json!({}),
        })
    }

    fn description(&self) -> &'static str {
        "A test action for discovery verification"
    }
}

#[tokio::test]
async fn test_discovery_types_compile() {
    let _loader = TestLoader;
    let _action = TestAction;
    let _params = EmptyParams;
    let _config = TestConfig;
    let _env = TestEnv;
}
