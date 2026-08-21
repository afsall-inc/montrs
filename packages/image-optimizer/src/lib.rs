//! Image optimization policy and bounded processing facade.

use montrs_image_core::{ImageError, ImageSpec};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OptimizerError {
    #[error(transparent)]
    Spec(#[from] ImageError),
    #[error("image source does not exist: {0}")]
    MissingSource(PathBuf),
    #[error("image source exceeds the maximum file size")]
    FileTooLarge,
}

#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    pub root: PathBuf,
    pub max_dimension: u32,
    pub max_file_size: u64,
}

impl OptimizerConfig {
    pub fn validate_spec(
        &self,
        spec: &ImageSpec,
    ) -> Result<PathBuf, OptimizerError> {
        spec.validate(self.max_dimension)?;
        let path = spec.resolve_under(&self.root)?;
        if !path.is_file() {
            return Err(OptimizerError::MissingSource(path));
        }
        let metadata = std::fs::metadata(&path)
            .map_err(|_| OptimizerError::MissingSource(path.clone()))?;
        if metadata.len() > self.max_file_size {
            return Err(OptimizerError::FileTooLarge);
        }
        Ok(path)
    }

    pub fn cache_path(
        &self,
        spec: &ImageSpec,
        cache_root: impl AsRef<Path>,
    ) -> PathBuf {
        let mut path =
            cache_root.as_ref().join(safe_cache_name(&spec.cache_key()));
        path.set_extension("img");
        path
    }
}

fn safe_cache_name(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' => {
                byte as char
            }
            _ => '_',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_names_are_filesystem_safe() {
        assert_eq!(safe_cache_name("hero.png:100"), "hero_png_100");
    }
}
