use crate::types::{HapticsProvider, ImpactStyle};

pub struct MobileHapticsProvider;

impl Default for MobileHapticsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MobileHapticsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl HapticsProvider for MobileHapticsProvider {
    fn vibrate(&self, _duration_ms: u32) {
        // TODO(#61): implement via platform-native bridge
        // Android: android.os.Vibrator via JNI (jni crate)
        // iOS: UIImpactFeedbackGenerator via objc2 FFI
    }

    fn vibrate_pattern(&self, _pattern: &[u32]) {
        // TODO(#61): implement via platform-native bridge
    }

    fn impact(&self, _style: ImpactStyle) {
        // TODO(#61): implement via platform-native bridge
    }

    fn selection_changed(&self) {
        // TODO(#61): implement via platform-native bridge
    }

    fn is_supported(&self) -> bool {
        cfg!(target_os = "android") || cfg!(target_os = "ios")
    }

    fn description(&self) -> &str {
        "Mobile haptics via platform-native APIs"
    }
}
