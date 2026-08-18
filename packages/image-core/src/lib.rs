//! Safe image specifications shared by UI and server optimizers.

use serde::{Deserialize, Serialize};
use std::path::{Component, Path};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ImageError {
    #[error("image dimensions must be non-zero")]
    InvalidDimensions,
    #[error("image dimensions exceed the configured limit")]
    DimensionsTooLarge,
    #[error("image quality must be between 1 and 100")]
    InvalidQuality,
    #[error("image source escapes the configured root")]
    UnsafeSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Original,
    Webp,
    Png,
    Jpeg,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSpec {
    pub source: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub quality: u8,
    pub format: ImageFormat,
}

impl ImageSpec {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            width: None,
            height: None,
            quality: 80,
            format: ImageFormat::Original,
        }
    }

    pub fn validate(&self, max_dimension: u32) -> Result<(), ImageError> {
        if self.quality == 0 || self.quality > 100 {
            return Err(ImageError::InvalidQuality);
        }
        for dimension in [self.width, self.height].into_iter().flatten() {
            if dimension == 0 {
                return Err(ImageError::InvalidDimensions);
            }
            if dimension > max_dimension {
                return Err(ImageError::DimensionsTooLarge);
            }
        }
        Ok(())
    }

    pub fn cache_key(&self) -> String {
        format!(
            "{}:{}:{}:{}:{:?}",
            self.source,
            self.width.map_or(0, |value| value),
            self.height.map_or(0, |value| value),
            self.quality,
            self.format
        )
    }

    pub fn resolve_under(&self, root: impl AsRef<Path>) -> Result<std::path::PathBuf, ImageError> {
        let path = Path::new(&self.source);
        if path.is_absolute() || path.components().any(|component| component == Component::ParentDir) {
            return Err(ImageError::UnsafeSource);
        }
        Ok(root.as_ref().join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_dimensions_and_quality() {
        let mut spec = ImageSpec::new("hero.png");
        spec.width = Some(800);
        assert!(spec.validate(1200).is_ok());
        spec.quality = 101;
        assert_eq!(spec.validate(1200), Err(ImageError::InvalidQuality));
    }

    #[test]
    fn rejects_parent_paths() {
        let spec = ImageSpec::new("../secret.png");
        assert_eq!(spec.resolve_under("public"), Err(ImageError::UnsafeSource));
    }
}
