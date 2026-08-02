//! montrs-motion: Animation library for MontRS.
//!
//! Inspired by Motion (Framer Motion) and Remotion. Provides spring physics,
//! tweening, keyframe interpolation, gestures, SVG path animation, and
//! an optional video creation pipeline.

pub mod spring;
pub mod tween;
pub mod keyframes;
pub mod value;
pub mod frame;
pub mod gesture;
pub mod svg;
pub mod css;
pub mod animated;

#[cfg(feature = "video")]
pub mod video;

pub use spring::Spring;
pub use tween::{Easing, Tween};
pub use keyframes::Keyframes;
pub use value::MotionValue;
pub use frame::FrameLoop;
pub use gesture::*;
pub use svg::*;
pub use css::*;
pub use animated::*;

#[cfg(feature = "video")]
pub use video::*;

/// Version of the montrs-motion crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");