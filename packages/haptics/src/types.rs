use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImpactStyle {
    Light,
    Medium,
    Heavy,
    Rigid,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HapticsTarget {
    Web,
    Desktop,
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticsConfig {
    pub enabled: bool,
    pub target: HapticsTarget,
}

impl Default for HapticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            target: HapticsTarget::Desktop,
        }
    }
}

pub trait HapticsProvider: Send + Sync {
    fn vibrate(&self, duration_ms: u32);
    fn vibrate_pattern(&self, pattern: &[u32]);
    fn impact(&self, style: ImpactStyle);
    fn selection_changed(&self);
    fn is_supported(&self) -> bool;
    fn description(&self) -> &str;
}
