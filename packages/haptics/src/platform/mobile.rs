// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
    fn vibrate(&self, duration_ms: u32) {
        #[cfg(target_os = "android")]
        {
            android_vibrate(duration_ms);
        }
        #[cfg(target_os = "ios")]
        {
            ios_vibrate(duration_ms);
        }
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            let _ = duration_ms;
        }
    }

    fn vibrate_pattern(&self, pattern: &[u32]) {
        #[cfg(target_os = "android")]
        {
            android_vibrate_pattern(pattern);
        }
        #[cfg(target_os = "ios")]
        {
            for &ms in pattern {
                ios_vibrate(ms);
            }
        }
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            let _ = pattern;
        }
    }

    fn impact(&self, style: ImpactStyle) {
        #[cfg(target_os = "android")]
        {
            android_impact(style);
        }
        #[cfg(target_os = "ios")]
        {
            ios_impact(style);
        }
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            let _ = style;
        }
    }

    fn selection_changed(&self) {
        #[cfg(target_os = "android")]
        {
            android_vibrate(10);
        }
        #[cfg(target_os = "ios")]
        {
            ios_selection_changed();
        }
    }

    fn is_supported(&self) -> bool {
        cfg!(target_os = "android") || cfg!(target_os = "ios")
    }

    fn description(&self) -> &str {
        "Mobile haptics via platform-native APIs"
    }
}

#[cfg(target_os = "android")]
fn android_vibrate(duration_ms: u32) {
    use jni::{
        JNIEnv,
        objects::{JClass, JObject, JValue},
        sys::{jlong, jobject},
    };

    let vm = jni::JavaVM::new().ok();
    if let Some(vm) = vm {
        if let Ok(mut env) = vm.attach_current_thread() {
            let activity = get_activity(&mut env);
            if let Some(activity) = activity {
                let vibrator = env.call_method(
                    activity,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[JValue::Object(
                        &env.new_string("vibrator").unwrap().into(),
                    )],
                );
                if let Ok(vibrator) = vibrator {
                    let _ = env.call_method(
                        vibrator.l().unwrap(),
                        "vibrate",
                        "(J)V",
                        &[JValue::Long(duration_ms as jlong)],
                    );
                }
            }
        }
    }
}

#[cfg(target_os = "android")]
fn android_vibrate_pattern(pattern: &[u32]) {
    use jni::{
        JNIEnv,
        objects::{JClass, JObject, JValue},
        sys::jlong,
    };

    let vm = jni::JavaVM::new().ok();
    if let Some(vm) = vm {
        if let Ok(mut env) = vm.attach_current_thread() {
            let activity = get_activity(&mut env);
            if let Some(activity) = activity {
                let vibrator = env.call_method(
                    activity,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[JValue::Object(
                        &env.new_string("vibrator").unwrap().into(),
                    )],
                );
                if let Ok(vibrator) = vibrator {
                    let pattern_arr: Vec<jlong> =
                        pattern.iter().map(|&ms| ms as jlong).collect();
                    let arr = env.new_long_array(pattern.len() as i32).ok();
                    if let Some(arr) = arr {
                        let _ =
                            env.set_long_array_region(&arr, 0, &pattern_arr);
                        let _ = env.call_method(
                            vibrator.l().unwrap(),
                            "vibrate",
                            "([JI)V",
                            &[JValue::Object(&arr.into()), JValue::Int(-1)],
                        );
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "android")]
fn android_impact(style: ImpactStyle) {
    use jni::{
        JNIEnv,
        objects::{JClass, JObject, JValue},
    };

    let ms = match style {
        ImpactStyle::Light => 10,
        ImpactStyle::Medium => 20,
        ImpactStyle::Heavy => 40,
        ImpactStyle::Rigid => 30,
        ImpactStyle::Soft => 15,
    };
    android_vibrate(ms);
}

#[cfg(target_os = "android")]
fn get_activity(env: &mut JNIEnv) -> Option<JObject> {
    // In a real app, this would get the Activity from the app context.
    // For now, try to get it from the thread's class loader.
    let cls = env.find_class("android/app/Activity").ok()?;
    Some(JObject::from(cls))
}

#[cfg(target_os = "ios")]
fn ios_vibrate(_duration_ms: u32) {
    use objc2::{msg_send, rc::autoreleasepool};
    use objc2_foundation::NSObject;

    autoreleasepool(|_| {
        let cls =
            objc2::runtime::AnyClass::get("UIImpactFeedbackGenerator").unwrap();
        let generator: *mut NSObject = unsafe { msg_send![cls, alloc] };
        if !generator.is_null() {
            let generator: *mut NSObject = unsafe {
                msg_send![generator, initWithStyle: 0] // UIImpactFeedbackStyleLight
            };
            let _: () = unsafe { msg_send![generator, prepare] };
            let _: () = unsafe { msg_send![generator, impactOccurred] };
        }
    });
}

#[cfg(target_os = "ios")]
fn ios_impact(style: ImpactStyle) {
    use objc2::{msg_send, rc::autoreleasepool};
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
            objc2::runtime::AnyClass::get("UIImpactFeedbackGenerator").unwrap();
        let generator: *mut NSObject = unsafe { msg_send![cls, alloc] };
        if !generator.is_null() {
            let generator: *mut NSObject =
                unsafe { msg_send![generator, initWithStyle: style_val] };
            let _: () = unsafe { msg_send![generator, prepare] };
            let _: () = unsafe { msg_send![generator, impactOccurred] };
        }
    });
}

#[cfg(target_os = "ios")]
fn ios_selection_changed() {
    use objc2::{msg_send, rc::autoreleasepool};
    use objc2_foundation::NSObject;

    autoreleasepool(|_| {
        let cls = objc2::runtime::AnyClass::get("UISelectionFeedbackGenerator")
            .unwrap();
        let generator: *mut NSObject = unsafe { msg_send![cls, new] };
        if !generator.is_null() {
            let _: () = unsafe { msg_send![generator, prepare] };
            let _: () = unsafe { msg_send![generator, selectionChanged] };
        }
    });
}
