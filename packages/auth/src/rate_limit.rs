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
