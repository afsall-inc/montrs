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

//! montrs-core/src/features.rs: Dynamic feature flags and user segmentation.
//! This file allows for runtime feature toggling and segment-based
//! targeting to support A/B testing and phased rollouts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for an individual feature flag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    /// Optional whitelist of segments that have access to this feature.
    pub segment_whitelist: Vec<String>,
}

/// Defines a group of users based on specific matching rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub rules: Vec<Rule>,
}

/// Evaluation rules for determining segment membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rule {
    /// Matches a specific user attribute.
    AttributeMatch { key: String, value: String },
    /// Matches a percentage of users deterministically via hashing.
    PercentageMatch(u8),
}

/// The feature manager responsible for evaluating flags and segments.
pub struct FeatureManager {
    flags: HashMap<String, FeatureFlag>,
    segments: HashMap<String, Segment>,
}

impl Default for FeatureManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureManager {
    /// Creates a new, empty FeatureManager.
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
            segments: HashMap::new(),
        }
    }

    /// Evaluates if a feature flag is enabled for a given user context.
    pub fn is_enabled(&self, flag_name: &str, user_ctx: &UserContext) -> bool {
        if let Some(flag) = self.flags.get(flag_name) {
            if !flag.enabled {
                return false;
            }
            // If no whitelist, feature is globally enabled.
            if flag.segment_whitelist.is_empty() {
                return true;
            }
            // Check if user is in any whitelisted segment.
            for segment_id in &flag.segment_whitelist {
                if self.is_in_segment(segment_id, user_ctx) {
                    return true;
                }
            }
        }
        false
    }

    /// Internal check to see if a user context satisfies a segment's rules.
    fn is_in_segment(&self, segment_id: &str, user_ctx: &UserContext) -> bool {
        if let Some(segment) = self.segments.get(segment_id) {
            for rule in &segment.rules {
                match rule {
                    Rule::AttributeMatch { key, value } => {
                        if user_ctx.attributes.get(key) != Some(value) {
                            return false;
                        }
                    }
                    Rule::PercentageMatch(pct) => {
                        // Deterministic hash-based percentage match
                        let hash = hash_string(&user_ctx.id);
                        if (hash % 100) as u8 >= *pct {
                            return false;
                        }
                    }
                }
            }
            return true;
        }
        false
    }
}

/// Context representing the current user for feature evaluation.
pub struct UserContext {
    pub id: String,
    pub attributes: HashMap<String, String>,
}

/// Simple deterministic string hashing for percentage rollouts.
fn hash_string(s: &str) -> u32 {
    let mut h = 0u32;
    for b in s.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    h
}

