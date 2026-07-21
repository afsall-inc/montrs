use crate::types::{HapticsProvider, ImpactStyle};

#[cfg(target_os = "windows")]
extern "system" {
    fn MessageBeep(uType: u32) -> i32;
}

pub struct DesktopHapticsProvider;

impl Default for DesktopHapticsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopHapticsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl HapticsProvider for DesktopHapticsProvider {
    fn vibrate(&self, _duration_ms: u32) {
        // TODO(#61): wire to montrs-desktop engine once available
        #[cfg(target_os = "windows")]
        unsafe {
            // Temporary: WinAPI MessageBeep for basic audible/tactile feedback
            MessageBeep(0xFFFFFFFF);
        }
        #[cfg(target_os = "macos")]
        {
            // Placeholder: will use NSHapticFeedbackManager via montrs-desktop engine
            // See montrs-desktop for the proper integration
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            // Linux/other: no universal haptics API without the desktop engine
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        // TODO(#61): wire to montrs-desktop engine once available
        for &ms in pattern {
            self.vibrate(ms);
        }
    }

    fn impact(&self, style: ImpactStyle) {
        // TODO(#61): wire to montrs-desktop engine once available
        let ms = match style {
            ImpactStyle::Light => 10,
            ImpactStyle::Medium => 20,
            ImpactStyle::Heavy => 40,
            ImpactStyle::Rigid => 30,
            ImpactStyle::Soft => 15,
        };
        self.vibrate(ms);
    }

    fn selection_changed(&self) {
        // TODO(#61): wire to montrs-desktop engine once available
        self.vibrate(5);
    }

    fn is_supported(&self) -> bool {
        cfg!(target_os = "macos") || cfg!(target_os = "windows")
    }

    fn description(&self) -> &str {
        "Desktop haptics via OS-native APIs (temporary until montrs-desktop \
         engine)"
    }
}
