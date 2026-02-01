//! montrs-core/src/env.rs: Typed environment variable management.
//! This file provides traits and implementations for accessing environment
//! variables in a type-safe and mockable manner.

use crate::AgentError;
use std::error::Error;
use std::fmt;

/// Errors that can occur when retrieving or parsing environment variables.
#[derive(Debug)]
pub enum EnvError {
    MissingKey(String),
    InvalidType(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::MissingKey(k) => write!(f, "Missing environment variable: {}", k),
            EnvError::InvalidType(k) => write!(f, "Invalid type for environment variable: {}", k),
        }
    }
}

impl Error for EnvError {}

impl AgentError for EnvError {
    fn error_code(&self) -> &'static str {
        match self {
            EnvError::MissingKey(_) => "ENV_MISSING_KEY",
            EnvError::InvalidType(_) => "ENV_INVALID_TYPE",
        }
    }

    fn explanation(&self) -> String {
        match self {
            EnvError::MissingKey(k) => format!("The application expected the environment variable '{}' to be set, but it was not found.", k),
            EnvError::InvalidType(k) => format!("The environment variable '{}' was found, but its value could not be parsed into the expected type.", k),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            EnvError::MissingKey(k) => vec![
                format!("Set the '{}' environment variable in your shell or .env file.", k),
                format!("Check if '{}' is correctly spelled in your configuration.", k),
            ],
            EnvError::InvalidType(k) => vec![
                format!("Ensure the value of '{}' matches the expected format (e.g., a number, boolean, or valid string).", k),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "env"
    }
}

/// Trait for types that can be initialized from an environment variable string.
pub trait FromEnv: Sized {
    fn from_env(val: String) -> Result<Self, String>;
}

impl FromEnv for String {
    fn from_env(val: String) -> Result<Self, String> {
        Ok(val)
    }
}

impl FromEnv for bool {
    fn from_env(val: String) -> Result<Self, String> {
        val.parse().map_err(|_| "bool".to_string())
    }
}

impl FromEnv for u16 {
    fn from_env(val: String) -> Result<Self, String> {
        val.parse().map_err(|_| "u16".to_string())
    }
}

impl FromEnv for u32 {
    fn from_env(val: String) -> Result<Self, String> {
        val.parse().map_err(|_| "u32".to_string())
    }
}

impl FromEnv for i32 {
    fn from_env(val: String) -> Result<Self, String> {
        val.parse().map_err(|_| "i32".to_string())
    }
}

/// Core trait for environment configuration providers.
/// Must be dyn-compatible (no generic methods directly).
pub trait EnvConfig: Send + Sync + 'static {
    /// Retrieves a raw string value for the given key.
    fn get_var(&self, key: &str) -> Result<String, EnvError>;

    /// Returns a list of expected environment variables and their descriptions.
    fn vars(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}

/// Extension trait to provide ergonomic typed access to environment variables.
pub trait EnvConfigExt: EnvConfig {
    /// Retrieves and parses an environment variable into the desired type T.
    fn get<T: FromEnv>(&self, key: &str) -> Result<T, EnvError> {
        let val = self.get_var(key)?;
        T::from_env(val).map_err(|_| EnvError::InvalidType(key.to_string()))
    }
}

impl<T: EnvConfig + ?Sized> EnvConfigExt for T {}

/// Default implementation of EnvConfig that reads from the system's environment.
#[derive(Clone)]
pub struct TypedEnv {
    // Standard implementation using std::env
}

impl EnvConfig for TypedEnv {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        std::env::var(key).map_err(|_| EnvError::MissingKey(key.to_string()))
    }
}
