pub mod desktop;
pub mod mobile;
#[cfg(feature = "web")]
pub mod web;

use crate::types::{HapticsConfig, HapticsProvider, ImpactStyle};

struct NoopHapticsProvider;

impl HapticsProvider for NoopHapticsProvider {
    fn vibrate(&self, _duration_ms: u32) {}
    fn vibrate_pattern(&self, _pattern: &[u32]) {}
    fn impact(&self, _style: ImpactStyle) {}
    fn selection_changed(&self) {}
    fn is_supported(&self) -> bool {
        false
    }
    fn description(&self) -> &str {
        "No-op (disabled by config)"
    }
}

pub fn create_haptics_provider(
    config: &HapticsConfig,
) -> Box<dyn HapticsProvider> {
    if !config.enabled {
        return Box::new(NoopHapticsProvider);
    }
    match config.target {
        crate::types::HapticsTarget::Web => {
            #[cfg(feature = "web")]
            {
                Box::new(web::WebHapticsProvider::new())
            }
            #[cfg(not(feature = "web"))]
            {
                Box::new(desktop::DesktopHapticsProvider::new())
            }
        }
        crate::types::HapticsTarget::Desktop => {
            Box::new(desktop::DesktopHapticsProvider::new())
        }
        crate::types::HapticsTarget::Mobile => {
            Box::new(mobile::MobileHapticsProvider::new())
        }
    }
}
