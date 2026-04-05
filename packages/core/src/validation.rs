use crate::AgentError;
use std::fmt;

/// Errors that can occur during schema validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Field length is less than the required minimum.
    MinLength {
        field: &'static str,
        min: usize,
        actual: usize,
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

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::MinLength { field, min, actual } => {
                write!(f, "{} is too short: {} (min {})", field, actual, min)
            }
            ValidationError::InvalidEmail { field } => {
                write!(f, "{} must be a valid email", field)
            }
            ValidationError::RegexMismatch { field, pattern } => {
                write!(f, "{} does not match pattern: {}", field, pattern)
            }
            ValidationError::Custom { field, message } => {
                write!(f, "{}: {}", field, message)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl AgentError for ValidationError {
    fn error_code(&self) -> &'static str {
        match self {
            ValidationError::MinLength { .. } => "VAL_MIN_LENGTH",
            ValidationError::InvalidEmail { .. } => "VAL_INVALID_EMAIL",
            ValidationError::RegexMismatch { .. } => "VAL_REGEX_MISMATCH",
            ValidationError::Custom { .. } => "VAL_CUSTOM",
        }
    }

    fn explanation(&self) -> String {
        match self {
            ValidationError::MinLength { field, min, actual } => format!(
                "The field '{}' has a length of {}, which is less than the required minimum of {}.",
                field, actual, min
            ),
            ValidationError::InvalidEmail { field } => format!(
                "The field '{}' does not contain a valid email address.",
                field
            ),
            ValidationError::RegexMismatch { field, pattern } => format!(
                "The field '{}' does not match the required pattern: {}.",
                field, pattern
            ),
            ValidationError::Custom { field, message } => {
                format!("Validation failed for field '{}': {}.", field, message)
            }
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            ValidationError::MinLength { min, .. } => vec![
                format!("Provide a value with at least {} characters.", min),
            ],
            ValidationError::InvalidEmail { .. } => vec![
                "Check the email address for typos and ensure it follows the standard format (e.g., user@example.com).".to_string(),
            ],
            ValidationError::RegexMismatch { pattern, .. } => vec![
                format!("Ensure the input matches the pattern: {}.", pattern),
            ],
            ValidationError::Custom { .. } => vec![
                "Review the custom validation logic or the input data to ensure it meets the requirements.".to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "validation"
    }
}

/// Trait for types that can be validated against a schema.
pub trait Validate {
    /// Validates the struct and returns a list of all validation errors found.
    fn validate(&self) -> Result<(), Vec<ValidationError>>;

    /// Returns the validation rules for this type, useful for agents to understand constraints.
    fn rules(&self) -> Vec<String> {
        Vec::new()
    }
}
