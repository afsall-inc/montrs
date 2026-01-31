//! The MontRS Framework - A full-stack Rust framework.

pub use montrs_core as core;

#[cfg(feature = "orm")]
pub use montrs_orm as orm;

#[cfg(feature = "validator")]
pub use montrs_validator as validator;

#[cfg(feature = "test")]
pub use montrs_test as test;

/// A convenience plate for importing the most commonly used types and traits.
pub mod prelude {
    pub use montrs_core::*;
    
    #[cfg(feature = "orm")]
    pub use montrs_orm::*;
    
    // montrs_validator is a proc-macro crate, we re-export its main macro
    #[cfg(feature = "validator")]
    pub use montrs_validator::Validator;
}
