//! montrs-ui: UI component library for MontRS.
//!
//! Provides Tailwind CSS macros, theming, and a component system inspired by
//! shadcn/ui. Re-exports `montrs-icons` for convenience.

pub use leptos;
pub use paste;
pub use tw_merge;

pub mod clx;
pub mod cn;
pub mod components;
pub mod theme;
pub mod utils;
pub mod variants;

pub use montrs_icons::*;

pub mod prelude {
    pub use crate::{
        clx::*,
        cn::*,
        components::*,
        theme::provider::{ThemeMode, ThemeProvider, toggle_theme, use_theme},
        variants::*,
    };
    pub use montrs_icons::*;
}
