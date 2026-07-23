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

use crate::AgentError;
use std::fmt;

/// Errors that can occur during validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorError {
    /// Field length is less than the required minimum.
    MinLength {
        field: &'static str,
        min: usize,
        actual: usize,
    },
    /// Field length exceeds the allowed maximum.
    MaxLength {
        field: &'static str,
        max: usize,
        actual: usize,
    },
    /// Numeric value is less than the required minimum.
    Min {
        field: &'static str,
        min: i64,
        actual: i64,
    },
    /// Numeric value exceeds the allowed maximum.
    Max {
        field: &'static str,
        max: i64,
        actual: i64,
    },
    /// Field does not contain a valid email format.
    InvalidEmail { field: &'static str },
    /// Field does not match the required regular expression.
    RegexMismatch {
        field: &'static str,
        pattern: &'static str,
    },
    /// A custom validation rule failed.
    Custom {
        field: &'static str,
        message: String,
    },
}

impl ValidatorError {
    /// Creates a new custom validation error.
    pub fn new(field: &'static str, message: impl Into<String>) -> Self {
        ValidatorError::Custom {
            field,
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidatorError::MinLength { field, min, actual } => {
                write!(f, "{field} is too short: {actual} (min {min})")
            }
            ValidatorError::MaxLength { field, max, actual } => {
                write!(f, "{field} is too long: {actual} (max {max})")
            }
            ValidatorError::Min { field, min, actual } => {
                write!(f, "{field} is too small: {actual} (min {min})")
            }
            ValidatorError::Max { field, max, actual } => {
                write!(f, "{field} is too large: {actual} (max {max})")
            }
            ValidatorError::InvalidEmail { field } => {
                write!(f, "{field} must be a valid email")
            }
            ValidatorError::RegexMismatch { field, pattern } => {
                write!(f, "{field} does not match pattern: {pattern}")
            }
            ValidatorError::Custom { field, message } => {
                write!(f, "{field}: {message}")
            }
        }
    }
}

impl std::error::Error for ValidatorError {}

impl AgentError for ValidatorError {
    fn error_code(&self) -> &'static str {
        match self {
            ValidatorError::MinLength { .. } => "VAL_MIN_LENGTH",
            ValidatorError::MaxLength { .. } => "VAL_MAX_LENGTH",
            ValidatorError::Min { .. } => "VAL_MIN",
            ValidatorError::Max { .. } => "VAL_MAX",
            ValidatorError::InvalidEmail { .. } => "VAL_INVALID_EMAIL",
            ValidatorError::RegexMismatch { .. } => "VAL_REGEX_MISMATCH",
            ValidatorError::Custom { .. } => "VAL_CUSTOM",
        }
    }

    fn explanation(&self) -> String {
        match self {
            ValidatorError::MinLength { field, min, actual } => format!(
                "The field '{field}' has a length of {actual}, which is less \
                 than the required minimum of {min}."
            ),
            ValidatorError::MaxLength { field, max, actual } => format!(
                "The field '{field}' has a length of {actual}, which exceeds \
                 the allowed maximum of {max}."
            ),
            ValidatorError::Min { field, min, actual } => format!(
                "The field '{field}' has a value of {actual}, which is less \
                 than the required minimum of {min}."
            ),
            ValidatorError::Max { field, max, actual } => format!(
                "The field '{field}' has a value of {actual}, which exceeds \
                 the allowed maximum of {max}."
            ),
            ValidatorError::InvalidEmail { field } => format!(
                "The field '{field}' does not contain a valid email address."
            ),
            ValidatorError::RegexMismatch { field, pattern } => format!(
                "The field '{field}' does not match the required pattern: \
                 {pattern}."
            ),
            ValidatorError::Custom { field, message } => {
                format!("Validation failed for field '{field}': {message}.")
            }
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            ValidatorError::MinLength { min, .. } => {
                vec![format!("Provide a value with at least {min} characters.")]
            }
            ValidatorError::MaxLength { max, .. } => {
                vec![format!("Provide a value with at most {max} characters.")]
            }
            ValidatorError::Min { min, .. } => {
                vec![format!("Provide a value greater than or equal to {min}.")]
            }
            ValidatorError::Max { max, .. } => {
                vec![format!("Provide a value less than or equal to {max}.")]
            }
            ValidatorError::InvalidEmail { .. } => vec![
                "Check the email address for typos and ensure it follows the \
                 standard format (e.g., user@example.com)."
                    .to_string(),
            ],
            ValidatorError::RegexMismatch { pattern, .. } => vec![format!(
                "Ensure the input matches the pattern: {pattern}."
            )],
            ValidatorError::Custom { .. } => vec![
                "Review the custom validation logic or the input data to \
                 ensure it meets the requirements."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "validation"
    }

    fn documentation_refs(&self) -> Vec<String> {
        vec!["packages/validator/docs/agent-guide".to_string()]
    }
}

/// Trait for types that can be validated.
pub trait Validator {
    /// Validates the struct and returns a list of all validation errors found.
    fn validate(&self) -> Result<(), Vec<ValidatorError>>;

    /// Returns the validation rules for this type, useful for agents to understand constraints.
    fn rules(&self) -> Vec<String> {
        Vec::new()
    }
}
