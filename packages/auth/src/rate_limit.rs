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

//! Sliding-window rate limiter for auth endpoints.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// In-memory sliding window rate limiter.
#[derive(Debug)]
pub struct RateLimiter {
    max: u32,
    window: Duration,
    hits: Mutex<HashMap<String, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new(max: u32, window_secs: u64) -> Self {
        Self {
            max,
            window: Duration::from_secs(window_secs.max(1)),
            hits: Mutex::new(HashMap::new()),
        }
    }

    /// Returns true if the key is allowed (under limit). Records a hit when allowed.
    pub fn check(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut hits = self.hits.lock().unwrap();
        let entry = hits.entry(key.to_string()).or_default();
        entry.retain(|t| now.duration_since(*t) < self.window);
        if entry.len() as u32 >= self.max {
            return false;
        }
        entry.push(now);
        true
    }

    /// Check without recording (peek).
    pub fn remaining(&self, key: &str) -> u32 {
        let now = Instant::now();
        let hits = self.hits.lock().unwrap();
        let count = hits
            .get(key)
            .map(|v| {
                v.iter()
                    .filter(|t| now.duration_since(**t) < self.window)
                    .count()
            })
            .unwrap_or(0) as u32;
        self.max.saturating_sub(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_under_limit() {
        let rl = RateLimiter::new(3, 60);
        assert!(rl.check("ip:1"));
        assert!(rl.check("ip:1"));
        assert!(rl.check("ip:1"));
        assert!(!rl.check("ip:1"));
    }

    #[test]
    fn separate_keys() {
        let rl = RateLimiter::new(1, 60);
        assert!(rl.check("a"));
        assert!(rl.check("b"));
        assert!(!rl.check("a"));
    }
}
