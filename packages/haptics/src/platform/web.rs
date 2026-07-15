use crate::types::{HapticsProvider, ImpactStyle};

pub struct WebHapticsProvider;

impl WebHapticsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl HapticsProvider for WebHapticsProvider {
    fn vibrate(&self, duration_ms: u32) {
        if let Some(nav) = web_sys::window().and_then(|w| w.navigator()) {
            let _ = nav.vibrate_with_duration(duration_ms);
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        if let Some(nav) = web_sys::window().and_then(|w| w.navigator()) {
            let js_arr = wasm_bindgen::JsValue::from(
                pattern.iter().map(|d| *d as f64).collect::<Vec<f64>>(),
            );
            let _ = nav.vibrate(&js_arr);
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
            .and_then(|w| w.navigator())
            .map(|n| n.vibrate(0).is_ok())
            .unwrap_or(false)
    }

    fn description(&self) -> &str {
        "Web Vibration API via navigator.vibrate()"
    }
}
