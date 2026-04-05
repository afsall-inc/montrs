//! The MontRS Framework - A full-stack Rust framework.

pub use montrs_core as core;

#[cfg(feature = "orm")]
pub use montrs_orm as orm;

#[cfg(feature = "schema")]
pub use montrs_schema as schema;

#[cfg(feature = "test")]
pub use montrs_test as test;

/// A convenience plate for importing the most commonly used types and traits.
pub mod prelude {
    pub use montrs_core::*;

    #[cfg(feature = "orm")]
    pub use montrs_orm::*;

    // montrs_schema is a proc-macro crate, we re-export its main macro
    #[cfg(feature = "schema")]
    pub use montrs_schema::Schema;
}
