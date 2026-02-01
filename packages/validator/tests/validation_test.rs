use montrs_core::{Validator, ValidatorError};
use montrs_validator::Validator;

#[derive(Validator)]
struct User {
    #[validator(min_len = 3)]
    username: String,
    #[validator(email)]
    email: String,
    #[validator(regex = r"^\d{4}-\d{2}-\d{2}$")]
    birth_date: String,
    #[validator(custom = "validate_custom")]
    status: String,
}

#[derive(Validator)]
struct Product {
    #[validator(min_len = 2, max_len = 10)]
    name: String,
    #[validator(min = 1, max = 100)]
    price: i32,
}

impl User {
    fn validate_custom(&self) -> Result<(), String> {
        if self.status == "forbidden" {
            Err("Status cannot be forbidden".to_string())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_validation_success() {
    let user = User {
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        birth_date: "1990-01-01".to_string(),
        status: "active".to_string(),
    };

    assert!(user.validate().is_ok());
}

#[test]
fn test_validation_failure_multiple_errors() {
    let user = User {
        username: "al".to_string(),         // too short
        email: "invalid-email".to_string(), // no @
        birth_date: "90-01-01".to_string(), // wrong format
        status: "forbidden".to_string(),    // custom error
    };

    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    assert_eq!(errors.len(), 4);

    assert!(matches!(
        errors[0],
        ValidatorError::MinLength {
            field: "username",
            min: 3,
            actual: 2
        }
    ));
    assert!(matches!(
        errors[1],
        ValidatorError::InvalidEmail { field: "email" }
    ));
    assert!(matches!(
        errors[2],
        ValidatorError::RegexMismatch {
            field: "birth_date",
            ..
        }
    ));
    assert!(matches!(
        errors[3],
        ValidatorError::Custom {
            field: "status",
            ..
        }
    ));
}

#[test]
fn test_regex_lazy_initialization() {
    let user = User {
        username: "bob".to_string(),
        email: "bob@example.com".to_string(),
        birth_date: "2000-12-31".to_string(),
        status: "active".to_string(),
    };

    // First call initializes regex
    assert!(user.validate().is_ok());
    // Second call uses already initialized regex
    assert!(user.validate().is_ok());
}

#[test]
fn test_product_validation() {
    let p = Product {
        name: "A".to_string(), // too short
        price: 0,              // too small
    };
    let errors = p.validate().unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(matches!(errors[0], ValidatorError::MinLength { .. }));
    assert!(matches!(errors[1], ValidatorError::Min { .. }));

    let p = Product {
        name: "Very long product name".to_string(), // too long
        price: 101,                                 // too large
    };
    let errors = p.validate().unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(matches!(errors[0], ValidatorError::MaxLength { .. }));
    assert!(matches!(errors[1], ValidatorError::Max { .. }));

    let p = Product {
        name: "Valid".to_string(),
        price: 50,
    };
    assert!(p.validate().is_ok());
}
