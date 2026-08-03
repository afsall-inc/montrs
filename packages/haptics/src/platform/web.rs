use crate::types::{HapticsProvider, ImpactStyle};

pub struct WebHapticsProvider;

impl Default for WebHapticsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WebHapticsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl HapticsProvider for WebHapticsProvider {
    fn vibrate(&self, duration_ms: u32) {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let _ = nav.vibrate_with_duration(duration_ms);
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let js_arr = wasm_bindgen::JsValue::from(
                pattern.iter().map(|d| *d as f64).collect::<Vec<f64>>(),
            );
            let _ = nav.vibrate_with_pattern(&js_arr);
        }
    }

    fn impact(&self, style: ImpactStyle) {
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
        self.vibrate(5);
    }

    fn is_supported(&self) -> bool {
        web_sys::window()
            .map(|w| w.navigator().vibrate_with_duration(0))
            .unwrap_or(false)
    }

    fn description(&self) -> &str {
        "Web Vibration API via navigator.vibrate()"
    }
}
