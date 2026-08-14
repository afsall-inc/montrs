//! Retry logic — exponential backoff for failing services.

use crate::config::RetryPolicy;
use tokio::time::{sleep, Duration};

/// Tracks retry state for a service.
#[derive(Debug, Default)]
pub struct RetryState {
    pub attempts: u32,
    pub last_attempt: Option<chrono::DateTime<chrono::Utc>>,
}

impl RetryState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether another retry is allowed under the policy.
    pub fn should_retry(&self, policy: &RetryPolicy) -> bool {
        self.attempts < policy.count
    }

    /// Compute the delay before the next attempt (with optional backoff).
    pub fn delay_for(&self, policy: &RetryPolicy) -> Duration {
        if policy.backoff {
            let exp = self.attempts.saturating_sub(1).min(10);
            let factor = 2u64.saturating_pow(exp);
            let secs = (policy.delay_secs * factor).min(policy.max_backoff_secs);
            Duration::from_secs(secs)
        } else {
            Duration::from_secs(policy.delay_secs)
        }
    }

    /// Record a failed attempt and (optionally) wait the policy delay.
    pub async fn record_failure(&mut self, policy: &RetryPolicy) {
        self.attempts += 1;
        self.last_attempt = Some(chrono::Utc::now());
        if self.should_retry(policy) {
            let delay = self.delay_for(policy);
            sleep(delay).await;
        }
    }

    pub fn reset(&mut self) {
        self.attempts = 0;
        self.last_attempt = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff_delay() {
        let policy = RetryPolicy {
            count: 5,
            delay_secs: 1,
            backoff: true,
            max_backoff_secs: 8,
        };
        let mut state = RetryState::new();
        state.attempts = 1;
        assert_eq!(state.delay_for(&policy), Duration::from_secs(1));
        state.attempts = 2;
        assert_eq!(state.delay_for(&policy), Duration::from_secs(2));
        state.attempts = 3;
        assert_eq!(state.delay_for(&policy), Duration::from_secs(4));
        state.attempts = 4;
        assert_eq!(state.delay_for(&policy).as_secs(), 8); // capped
    }

    #[tokio::test]
    async fn test_should_retry() {
        let policy = RetryPolicy {
            count: 2,
            ..Default::default()
        };
        let mut state = RetryState::new();
        assert!(state.should_retry(&policy));
        state.attempts = 1;
        assert!(state.should_retry(&policy));
        state.attempts = 2;
        assert!(!state.should_retry(&policy));
        let _ = policy;
    }
}