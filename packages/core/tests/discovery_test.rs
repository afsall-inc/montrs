use async_trait::async_trait;
use montrs_core::{
    Action, ActionCtx, ActionResponse, AppConfig, Loader, LoaderCtx, LoaderResponse,
};

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

struct TestLoader;

#[async_trait]
impl Loader<TestConfig> for TestLoader {
    async fn call(
        &self,
        _ctx: LoaderCtx<TestConfig>,
    ) -> Result<LoaderResponse, Box<dyn std::error::Error + Send + Sync>> {
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
impl Action<TestConfig> for TestAction {
    async fn call(
        &self,
        _input: serde_json::Value,
        _ctx: ActionCtx<TestConfig>,
    ) -> Result<ActionResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ActionResponse {
            data: serde_json::json!({}),
        })
    }

    fn description(&self) -> &'static str {
        "A test action for discovery verification"
    }
}
