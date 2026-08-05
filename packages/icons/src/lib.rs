//! montrs-icons: Lucide icons for MontRS applications.
//!
//! Provides 1600+ icons as Leptos components, grouped into 42 category features
//! for selective compilation. Prefer the per-icon convenience components
//! (e.g. [`SearchIcon`]) over the generic [`Icon`] component for static usage.
//! Use the [`glyph!`] macro with the generic [`Icon`] when you need a dynamic
//! or reactive glyph.

pub mod glyph;
pub mod glyph_impl;
pub mod icon;
pub mod registry;

#[cfg(feature = "animated")]
pub mod animated;

#[cfg(feature = "animated")]
pub use animated::AnimatedIcon;
pub use glyph::Glyph;
pub use icon::{CustomIcon, Icon};
pub use registry::*;

/// Re-export strum traits for iterating/looking up icons.
pub mod strum {
    pub use ::strum::{EnumProperty, IntoEnumIterator};
}

/// Shorthand for constructing a [`Glyph`] variant.
///
/// This lets you avoid typing `Glyph::` when using the generic [`Icon`]
/// component:
///
/// ```rust,ignore
/// use montrs_icons::{glyph, Icon};
///
/// view! { <Icon glyph=glyph!(Search) /> }
/// ```
///
/// This is equivalent to `Glyph::Search`. Prefer the per-icon convenience
/// components (e.g. `<SearchIcon />`) for static usage; reach for this
/// macro when you need a dynamic or reactive glyph.
#[macro_export]
macro_rules! glyph {
    ($name:ident) => {
        $crate::Glyph::$name
    };
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
