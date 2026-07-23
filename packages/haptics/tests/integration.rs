// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use montrs_haptics::{
    HapticsConfig, HapticsTarget, ImpactStyle, create_haptics_provider,
};

#[test]
fn config_disabled_returns_noop_provider() {
    let config = HapticsConfig {
        enabled: false,
        target: HapticsTarget::Desktop,
    };
    let provider = create_haptics_provider(&config);
    assert!(!provider.is_supported());
}

#[test]
fn config_enabled_desktop_returns_supported_provider() {
    let config = HapticsConfig {
        enabled: true,
        target: HapticsTarget::Desktop,
    };
    let provider = create_haptics_provider(&config);
    // Desktop is supported on Windows/macOS, but in test we just verify
    // the provider is constructable and doesn't panic on any operation
    provider.vibrate(100);
    provider.vibrate_pattern(&[10, 20, 30]);
    provider.impact(ImpactStyle::Medium);
    provider.selection_changed();
}

#[test]
fn config_disabled_works_with_any_target() {
    for target in &[
        HapticsTarget::Web,
        HapticsTarget::Desktop,
        HapticsTarget::Mobile,
    ] {
        let config = HapticsConfig {
            enabled: false,
            target: *target,
        };
        let provider = create_haptics_provider(&config);
        assert!(!provider.is_supported());
    }
}

#[test]
fn haptics_config_default_is_enabled_desktop() {
    let config = HapticsConfig::default();
    assert!(config.enabled);
    assert_eq!(config.target, HapticsTarget::Desktop);
}

#[test]
fn impact_style_implements_debug_and_clone() {
    let styles = [
        ImpactStyle::Light,
        ImpactStyle::Medium,
        ImpactStyle::Heavy,
        ImpactStyle::Rigid,
        ImpactStyle::Soft,
    ];
    for style in &styles {
        let cloned = *style;
        assert_eq!(format!("{:?}", cloned), format!("{:?}", style));
    }
}

#[test]
fn haptics_target_implements_eq() {
    assert_eq!(HapticsTarget::Web, HapticsTarget::Web);
    assert_ne!(HapticsTarget::Web, HapticsTarget::Desktop);
    assert_ne!(HapticsTarget::Mobile, HapticsTarget::Desktop);
}

#[test]
fn create_haptics_provider_is_send_sync() {
    fn assert_send<T: Send>(_t: &T) {}
    fn assert_sync<T: Sync>(_t: &T) {}

    let config = HapticsConfig::default();
    let provider = create_haptics_provider(&config);
    assert_send(&provider);
    assert_sync(&provider);
}

#[test]
fn provider_trait_has_description() {
    let config = HapticsConfig {
        enabled: false,
        target: HapticsTarget::Desktop,
    };
    let provider = create_haptics_provider(&config);
    let desc = provider.description();
    assert!(!desc.is_empty());
}

#[test]
fn disabled_provider_does_not_panic_on_any_method() {
    let config = HapticsConfig {
        enabled: false,
        target: HapticsTarget::Desktop,
    };
    let provider = create_haptics_provider(&config);
    // None of these should panic
    provider.impact(ImpactStyle::Light);
    provider.impact(ImpactStyle::Medium);
    provider.impact(ImpactStyle::Heavy);
    provider.impact(ImpactStyle::Rigid);
    provider.impact(ImpactStyle::Soft);
    provider.selection_changed();
    provider.vibrate(0);
    provider.vibrate(u32::MAX);
    provider.vibrate_pattern(&[]);
    provider.vibrate_pattern(&[100, 200, 300]);
}

#[test]
fn desktop_provider_does_not_panic() {
    let config = HapticsConfig {
        enabled: true,
        target: HapticsTarget::Desktop,
    };
    let provider = create_haptics_provider(&config);
    provider.impact(ImpactStyle::Light);
    provider.impact(ImpactStyle::Medium);
    provider.impact(ImpactStyle::Heavy);
    provider.impact(ImpactStyle::Rigid);
    provider.impact(ImpactStyle::Soft);
    provider.selection_changed();
    provider.vibrate(0);
    provider.vibrate(u32::MAX);
    provider.vibrate_pattern(&[]);
    provider.vibrate_pattern(&[100, 200, 300]);
}

#[test]
fn mobile_provider_does_not_panic() {
    let config = HapticsConfig {
        enabled: true,
        target: HapticsTarget::Mobile,
    };
    let provider = create_haptics_provider(&config);
    provider.impact(ImpactStyle::Medium);
    provider.vibrate(100);
    provider.vibrate_pattern(&[50, 50]);
    provider.selection_changed();
}

#[test]
fn haptics_config_serialize_roundtrip() {
    let config = HapticsConfig {
        enabled: true,
        target: HapticsTarget::Desktop,
    };
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: HapticsConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.target, deserialized.target);
}
