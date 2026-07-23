// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! montrs-core/src/env.rs: Typed environment variable management.
//! This file provides traits and implementations for accessing environment
//! variables in a type-safe and mockable manner.

use crate::AgentError;
use std::{error::Error, fmt};

/// Errors that can occur when retrieving or parsing environment variables.
#[derive(Debug)]
pub enum EnvError {
    MissingKey(String),
    InvalidType(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::MissingKey(k) => {
                write!(f, "Missing environment variable: {k}")
            }
            EnvError::InvalidType(k) => {
                write!(f, "Invalid type for environment variable: {k}")
            }
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
            EnvError::MissingKey(k) => format!(
                "The application expected the environment variable '{k}' to \
                 be set, but it was not found."
            ),
            EnvError::InvalidType(k) => format!(
                "The environment variable '{k}' was found, but its value \
                 could not be parsed into the expected type."
            ),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            EnvError::MissingKey(k) => vec![
                format!(
                    "Set the '{k}' environment variable in your shell or .env \
                     file."
                ),
                format!(
                    "Check if '{k}' is correctly spelled in your \
                     configuration."
                ),
            ],
            EnvError::InvalidType(k) => vec![format!(
                "Ensure the value of '{k}' matches the expected format (e.g., \
                 a number, boolean, or valid string)."
            )],
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

