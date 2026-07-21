pub mod platform;
pub mod types;

pub use platform::create_haptics_provider;
pub use types::*;

pub const DESCRIPTION: &str =
    "Cross-platform haptic feedback for web, desktop, and mobile";
