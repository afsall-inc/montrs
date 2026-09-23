// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Simulated devices for responsive testing.
//!
//! A [`DeviceProfile`] is the input to a responsive check: its viewport width
//! (and optional container width) drive breakpoint resolution. The catalog
//! covers common phones, tablets, and desktops; [`DeviceProfile::from_observed`]
//! creates a profile from a real client so a developer's own device can be
//! tested and recorded.

use crate::layout::Viewport;

/// A coarse device class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCategory {
    /// Small phones and foldables.
    Phone,
    /// Tablets.
    Tablet,
    /// Laptops.
    Laptop,
    /// Desktop monitors.
    Desktop,
    /// An unknown/detected device.
    Unknown,
}

impl DeviceCategory {
    /// Human-readable label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Phone => "phone",
            Self::Tablet => "tablet",
            Self::Laptop => "laptop",
            Self::Desktop => "desktop",
            Self::Unknown => "unknown",
        }
    }

    /// Infer a category from a viewport width.
    pub fn from_width(width: f32) -> Self {
        if width < 640.0 {
            Self::Phone
        } else if width < 1024.0 {
            Self::Tablet
        } else if width < 1440.0 {
            Self::Laptop
        } else {
            Self::Desktop
        }
    }
}

/// A device viewport used for responsive testing.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceProfile {
    /// Stable identifier (e.g. `iphone-15`).
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Landscape-independent width in CSS pixels.
    pub width: f32,
    /// Height in CSS pixels.
    pub height: f32,
    /// Device pixel ratio.
    pub dpr: f32,
    /// Whether the device starts in portrait orientation.
    pub portrait: bool,
    /// Font scale (browser/user zoom).
    pub font_scale: f32,
    /// Device class.
    pub category: DeviceCategory,
}

impl DeviceProfile {
    /// Build a custom device profile.
    pub fn custom(
        id: impl Into<String>,
        name: impl Into<String>,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            width,
            height,
            dpr: 1.0,
            portrait: true,
            font_scale: 1.0,
            category: DeviceCategory::from_width(width),
        }
    }

    /// Build a profile from an observed client (`from_observed`).
    ///
    /// Used when a real device connects to the dev server: its dimensions are
    /// captured so it can be benchmarked and added to the matrix.
    pub fn from_observed(width: f32, height: f32, dpr: f32) -> Self {
        Self {
            id: format!(
                "observed-{}x{}",
                width.round() as i64,
                height.round() as i64
            ),
            name: format!(
                "Observed device {}×{} @{}x",
                width.round() as i64,
                height.round() as i64,
                dpr
            ),
            width,
            height,
            dpr,
            portrait: height >= width,
            font_scale: 1.0,
            category: DeviceCategory::from_width(width),
        }
    }

    /// Set the pixel ratio.
    pub fn with_dpr(mut self, dpr: f32) -> Self {
        self.dpr = dpr;
        self
    }

    /// Set the font scale.
    pub fn with_font_scale(mut self, scale: f32) -> Self {
        self.font_scale = scale;
        self
    }

    /// The viewport for this device's current width.
    pub fn viewport(&self) -> Viewport {
        Viewport::new(self.width, self.height)
    }

    /// The same device rotated.
    pub fn rotated(&self) -> Self {
        let mut rotated = self.clone();
        std::mem::swap(&mut rotated.width, &mut rotated.height);
        rotated.portrait = !self.portrait;
        rotated.id = format!("{}-rotated", self.id);
        rotated
    }
}

macro_rules! device {
    ($fn_name:ident, $id:literal, $name:literal, $w:expr, $h:expr, $dpr:expr, $cat:expr) => {
        /// A built-in device profile.
        pub fn $fn_name() -> DeviceProfile {
            DeviceProfile {
                id: $id.to_string(),
                name: $name.to_string(),
                width: $w,
                height: $h,
                dpr: $dpr,
                portrait: $h >= $w,
                font_scale: 1.0,
                category: $cat,
            }
        }
    };
}

device!(
    iphone_se,
    "iphone-se",
    "iPhone SE",
    375.0,
    667.0,
    2.0,
    DeviceCategory::Phone
);
device!(
    iphone_15,
    "iphone-15",
    "iPhone 15",
    393.0,
    852.0,
    3.0,
    DeviceCategory::Phone
);
device!(
    pixel_7,
    "pixel-7",
    "Pixel 7",
    412.0,
    915.0,
    2.625,
    DeviceCategory::Phone
);
device!(
    foldable,
    "foldable",
    "Foldable (folded)",
    344.0,
    882.0,
    3.0,
    DeviceCategory::Phone
);
device!(
    ipad_mini,
    "ipad-mini",
    "iPad mini",
    744.0,
    1133.0,
    2.0,
    DeviceCategory::Tablet
);
device!(
    ipad_pro,
    "ipad-pro",
    "iPad Pro 12.9",
    1024.0,
    1366.0,
    2.0,
    DeviceCategory::Tablet
);
device!(
    laptop_1280,
    "laptop-1280",
    "Laptop 1280",
    1280.0,
    800.0,
    2.0,
    DeviceCategory::Laptop
);
device!(
    desktop_1920,
    "desktop-1920",
    "Desktop 1920",
    1920.0,
    1080.0,
    1.0,
    DeviceCategory::Desktop
);

/// The default responsive test matrix.
pub fn standard_devices() -> Vec<DeviceProfile> {
    vec![
        iphone_se(),
        iphone_15(),
        pixel_7(),
        foldable(),
        ipad_mini(),
        ipad_pro(),
        laptop_1280(),
        desktop_1920(),
    ]
}

/// The narrowest standard width (a useful lower bound for sweeps).
pub fn min_standard_width() -> f32 {
    320.0
}

/// The widest standard width (a useful upper bound for sweeps).
pub fn max_standard_width() -> f32 {
    1920.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_consistent() {
        let devices = standard_devices();
        assert!(devices.len() >= 8);
        for d in &devices {
            assert!(d.width > 0.0 && d.height > 0.0);
            assert!(d.dpr > 0.0);
            assert_eq!(d.viewport().width, d.width);
        }
    }

    #[test]
    fn observed_device_is_recorded() {
        let d = DeviceProfile::from_observed(390.0, 844.0, 3.0);
        assert_eq!(d.id, "observed-390x844");
        assert_eq!(d.category, DeviceCategory::Phone);
        assert!(d.portrait);
    }

    #[test]
    fn categories_follow_width() {
        assert_eq!(DeviceCategory::from_width(375.0), DeviceCategory::Phone);
        assert_eq!(DeviceCategory::from_width(800.0), DeviceCategory::Tablet);
        assert_eq!(DeviceCategory::from_width(1280.0), DeviceCategory::Laptop);
        assert_eq!(DeviceCategory::from_width(1920.0), DeviceCategory::Desktop);
    }

    #[test]
    fn rotation_swaps_dimensions() {
        let d = iphone_15();
        let r = d.rotated();
        assert_eq!(r.width, d.height);
        assert_eq!(r.height, d.width);
        assert!(!r.portrait);
    }
}
