//! montrs-core: The core architectural engine for MontRS.
//!
//! This crate provides the foundational traits and structs that define how a MontRS
//! application is structured, initialized, and executed. It leverages Leptos 0.8
//! for fine-grained reactivity and provides a modular system for composing
//! complex applications.

pub mod env;
pub mod features;
pub mod limiter;
pub mod router;
pub mod validation;

use async_trait::async_trait;
pub use env::{EnvConfig, EnvConfigExt, EnvError, FromEnv, TypedEnv};
pub use features::{FeatureFlag, FeatureManager, Rule, Segment, UserContext};
pub use leptos::prelude::*;
pub use limiter::{GovernorLimiter, Limiter};
pub use router::{
    ActionResponse, LoaderResponse, Route, RouteAction, RouteContext,
    RouteError, RouteLoader, RouteParams, RouteView, Router,
};
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, sync::Arc};
pub use validation::{Validator, ValidatorError};

/// A trait for errors that provide agent-accessible metadata.
pub trait AgentError: StdError {
    /// A stable identifier for the error type.
    fn error_code(&self) -> &'static str;

    /// A structured explanation of the error, its cause, and context.
    fn explanation(&self) -> String;

    /// Suggested fixes or remediation steps.
    fn suggested_fixes(&self) -> Vec<String> {
        Vec::new()
    }

    /// The subsystem or package where the error originated.
    fn subsystem(&self) -> &'static str {
        "core"
    }

    /// Optional raw compiler error if applicable.
    fn rustc_error(&self) -> Option<String> {
        None
    }

    /// References to specific rules or documentation sections.
    /// Returns a list of keys that can be looked up in documentation_snippets.
    fn documentation_refs(&self) -> Vec<String> {
        Vec::new()
    }
}

/// The execution environment context for the application.
/// Used to differentiate logic between server-side rendering, WASM hydration,
/// and other deployment targets like Edge or Mobile.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Target {
    /// Server-side rendering (SSR) context.
    Server,
    /// Client-side WASM hydration or CSR context.
    Wasm,
    /// Edge computing environments (e.g., Cloudflare Workers).
    Edge,
    /// Desktop applications (e.g., via Tauri).
    Desktop,
    /// Android mobile platform.
    MobileAndroid,
    /// iOS mobile platform.
    MobileIos,
}

/// The unit of composition in MontRS.
///
/// A `Plate` encapsulates a logical piece of functionality (e.g., Auth, Users, Blog).
/// It provides hooks for initialization, context provision, and route registration.
/// Plates are designed to be portable across different `Target` environments.
#[async_trait]
pub trait Plate<C: AppConfig>: Send + Sync + 'static {
    /// Returns a static name for the plate, used for logging and debugging.
    fn name(&self) -> &'static str;

    /// Returns a description of what this plate does, useful for agents.
    fn description(&self) -> &'static str {
        ""
    }

    /// Returns key-value metadata for the plate.
    fn metadata(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }

    /// Returns the names of plates this plate depends on.
    /// This is used to verify initialization order and provide architectural context to agents.
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// The primary initialization point for a plate.
    ///
    /// This is called during the application bootstrap process. It should be used to:
    /// 1. Initialize local resources (database connections, etc.)
    /// 2. Provide Leptos contexts using `provide_context`.
    /// 3. Perform any required async setup.
    async fn init(
        &self,
        ctx: &mut PlateContext<C>,
    ) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// Register routes for this plate.
    ///
    /// This allows plates to define their own URL structure and link them to
    /// specific Loaders and Actions.
    fn register_routes(&self, _router: &mut Router<C>) {}
}

/// Dynamic context provided to plates during their `init` phase.
/// Includes access to the global application configuration and environment.
pub struct PlateContext<'a, C: AppConfig> {
    /// The strongly-typed application configuration.
    pub config: &'a C,
    /// The environment variable provider.
    pub env: &'a dyn EnvConfig,
}

/// Defining the "Shape" of the application.
///
/// Every MontRS application must implement `AppConfig` to define its
/// custom config structure, error handling, and environment requirements.
pub trait AppConfig: Sized + Send + Sync + Clone + 'static {
    /// The top-level error type used throughout the application.
    type Error: StdError + Send + Sync;
    /// The strongly-typed environment configuration.
    type Env: EnvConfig + Clone;

    /// Returns key-value metadata for the application.
    fn metadata(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}

/// The deterministic blueprint of a MontRS application.
///
/// `AppSpec` contains everything needed to boot the application: configuration,
/// registered plates, the environment, and the routing table. It is the single
/// source of truth for the application's structure.
#[derive(Clone)]
pub struct AppSpec<C: AppConfig> {
    /// Global application configuration.
    pub config: C,
    /// List of registered functional plates.
    pub plates: Vec<Arc<dyn Plate<C>>>,
    /// Resolved environment configuration.
    pub env: C::Env,
    /// The centralized routing table.
    pub router: Router<C>,
    /// The current execution target.
    pub target: Target,
}

/// A serializable version of AppSpec for external consumption (e.g., by agents).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSpecExport {
    pub name: String,
    pub target: Target,
    pub plates: Vec<PlateMetadata>,
    pub router: crate::router::RouterSpec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlateMetadata {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl<C: AppConfig> AppSpec<C> {
    /// Exports the application specification to a serializable format.
    pub fn export_spec(&self, app_name: &str) -> AppSpecExport {
        AppSpecExport {
            name: app_name.to_string(),
            target: self.target,
            plates: self
                .plates
                .iter()
                .map(|m| PlateMetadata {
                    name: m.name().to_string(),
                    description: m.description().to_string(),
                    dependencies: m
                        .dependencies()
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    metadata: m.metadata(),
                })
                .collect(),
            router: self.router.spec(),
        }
    }

    /// Creates a new, empty AppSpec with required config and environment.
    pub fn new(config: C, env: C::Env) -> Self {
        Self {
            config,
            plates: Vec::new(),
            env,
            router: Router::new(),
            target: Target::Server,
        }
    }

    /// Builder method to add a plate to the specification.
    pub fn with_plate(mut self, plate: impl Plate<C>) -> Self {
        self.plates.push(Arc::new(plate));
        self
    }

    /// Builder method to set the deployment target.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }

    /// Boots the application and mounts it to the document body.
    ///
    /// Inside this method:
    /// 1. The global config and env are provided as Leptos contexts.
    /// 2. All registered plates are initialized sequentially.
    /// 3. The `main_view` is rendered as the application root.
    pub fn mount<F, IV>(self, main_view: F)
    where
        F: FnOnce() -> IV + 'static,
        IV: IntoView + 'static,
    {
        let config = self.config;
        let env = self.env;
        let plates = self.plates;

        leptos::mount::mount_to_body(move || {
            // Provide global application context for easy access via use_context().
            provide_context(config.clone());
            provide_context(env.clone());

            // Initialize plates.
            for plate in plates {
                println!("Booting plate: {}", plate.name());
                // In v0.1, we acknowledge the async balance; true async init
                // typically happens outside the reactive loop or via Resources.
            }

            main_view()
        });
    }
}
