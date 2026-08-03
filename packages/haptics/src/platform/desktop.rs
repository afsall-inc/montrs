use crate::types::{HapticsProvider, ImpactStyle};

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
        #[cfg(target_os = "windows")]
        {
            extern "system" {
                fn MessageBeep(uType: u32) -> i32;
            }
            unsafe {
                MessageBeep(0xFFFFFFFF);
            }
        }
        #[cfg(target_os = "macos")]
        {
            use objc2::{msg_send, rc::autoreleasepool, runtime::Sel};
            use objc2_foundation::NSObject;

            autoreleasepool(|_| {
                let cls =
                    objc2::runtime::AnyClass::get("NSHapticFeedbackManager")
                        .unwrap();
                let performer: *mut NSObject =
                    unsafe { msg_send![cls, defaultPerformer] };
                if !performer.is_null() {
                    let _: () = unsafe {
                        msg_send![performer, performFeedbackPattern: 0]
                    };
                }
            });
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        for &ms in pattern {
            self.vibrate(ms);
        }
    }

    fn impact(&self, style: ImpactStyle) {
        #[cfg(target_os = "macos")]
        {
            use objc2::{msg_send, rc::autoreleasepool, runtime::Sel};
            use objc2_foundation::NSObject;

            let style_val = match style {
                ImpactStyle::Light => 0,
                ImpactStyle::Medium => 1,
                ImpactStyle::Heavy => 2,
                ImpactStyle::Rigid => 0,
                ImpactStyle::Soft => 0,
            };

            autoreleasepool(|_| {
                let cls =
                    objc2::runtime::AnyClass::get("NSHapticFeedbackManager")
                        .unwrap();
                let performer: *mut NSObject =
                    unsafe { msg_send![cls, defaultPerformer] };
                if !performer.is_null() {
                    let _: () = unsafe {
                        msg_send![performer, performFeedbackPattern: style_val]
                    };
                }
            });
        }
        #[cfg(not(target_os = "macos"))]
        {
            let ms = match style {
                ImpactStyle::Light => 10,
                ImpactStyle::Medium => 20,
                ImpactStyle::Heavy => 40,
                ImpactStyle::Rigid => 30,
                ImpactStyle::Soft => 15,
            };
            self.vibrate(ms);
        }
    }

    fn selection_changed(&self) {
        #[cfg(target_os = "macos")]
        {
            use objc2::{msg_send, rc::autoreleasepool, runtime::Sel};
            use objc2_foundation::NSObject;

            autoreleasepool(|_| {
                let cls =
                    objc2::runtime::AnyClass::get("NSHapticFeedbackManager")
                        .unwrap();
                let performer: *mut NSObject =
                    unsafe { msg_send![cls, defaultPerformer] };
                if !performer.is_null() {
                    let _: () = unsafe {
                        msg_send![performer, performFeedbackPattern: 1]
                    };
                }
            });
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.vibrate(5);
        }
    }

    fn is_supported(&self) -> bool {
        cfg!(target_os = "macos") || cfg!(target_os = "windows")
    }

    fn description(&self) -> &str {
        "Desktop haptics via OS-native APIs"
    }
}
