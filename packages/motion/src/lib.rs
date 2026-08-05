//! montrs-motion: Animation library for MontRS.
//!
//! Inspired by Motion (Framer Motion) and Remotion. Provides spring physics,
//! tweening, keyframe interpolation, gestures, SVG path animation, and
//! an optional video creation pipeline.

pub mod animated;
pub mod css;
pub mod frame;
pub mod gesture;
pub mod keyframes;
pub mod spring;
pub mod svg;
pub mod tween;
pub mod value;

#[cfg(feature = "video")]
pub mod video;

pub use animated::*;
pub use css::*;
pub use frame::FrameLoop;
pub use gesture::*;
pub use keyframes::Keyframes;
pub use spring::Spring;
pub use svg::*;
pub use tween::{Easing, Tween};
pub use value::MotionValue;
#[cfg(feature = "video")]
pub use video::*;

/// Version of the montrs-motion crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
