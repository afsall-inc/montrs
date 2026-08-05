//! The MontRS Framework - A full-stack Rust framework.

pub use montrs_core as core;
pub use montrs_platform as platform;
#[cfg(feature = "desktop")]
pub use montrs_desktop as desktop;
#[cfg(feature = "haptics")]
pub use montrs_haptics as haptics;
#[cfg(feature = "icons")]
pub use montrs_icons as icons;
#[cfg(feature = "mobile")]
pub use montrs_mobile as mobile;
#[cfg(feature = "motion")]
pub use montrs_motion as motion;
#[cfg(feature = "orm")]
pub use montrs_orm as orm;
#[cfg(feature = "renderer")]
pub use montrs_renderer as renderer;
#[cfg(feature = "test")]
pub use montrs_test as test;
#[cfg(feature = "ui")]
pub use montrs_ui as ui;
#[cfg(feature = "validator")]
pub use montrs_validator as validator;

/// A convenience plate for importing the most commonly used types and traits.
pub mod prelude {
    pub use montrs_core::*;
    pub use montrs_platform::{PlatformAdapter, Target};
    #[cfg(feature = "haptics")]
    pub use montrs_haptics::{HapticsConfig, HapticsProvider, ImpactStyle};
    #[cfg(feature = "icons")]
    pub use montrs_icons::*;
    #[cfg(feature = "motion")]
    pub use montrs_motion::*;
    #[cfg(feature = "orm")]
    pub use montrs_orm::*;
    #[cfg(feature = "ui")]
    pub use montrs_ui::prelude::*;
    // montrs_validator is a proc-macro crate, we re-export its main macro
    #[cfg(feature = "validator")]
    pub use montrs_validator::Validator;
}
