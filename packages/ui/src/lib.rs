//! montrs-ui: UI component library for MontRS.
//!
//! Provides Tailwind CSS macros, theming, and a component system inspired by
//! shadcn/ui. Re-exports `montrs-icons` for convenience.

pub use leptos;
pub use paste;
pub use tw_merge;

pub mod cn;
pub mod clx;
pub mod utils;
pub mod variants;
pub mod theme;
pub mod components;

pub use montrs_icons::*;

pub mod prelude {
    pub use crate::cn::*;
    pub use crate::clx::*;
    pub use crate::variants::*;
    pub use crate::components::*;
    pub use crate::theme::provider::{ThemeMode, ThemeProvider, use_theme, toggle_theme};
    pub use montrs_icons::*;
}