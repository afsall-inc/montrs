// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic time for tests.
//!
//! [`Clock`] abstracts "now" and "sleep" so services, retries, rate limiters,
//! and motion timelines can be driven by a [`TestClock`] that only moves when
//! the test tells it to — no wall clock, no flakiness.

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    time::Duration,
};

/// A source of monotonic time and a sleep primitive.
pub trait Clock: Send + Sync + 'static {
    /// Time elapsed since this clock's epoch.
    fn now(&self) -> Duration;

    /// Resolve once `duration` of virtual time has passed.
    fn sleep(
        &self,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;

    /// Upcast for downcasting to a concrete clock (e.g. [`TestClock`]).
    fn as_any(&self) -> &dyn std::any::Any;
}

/// A [`Clock`] backed by the real system monotonic clock.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock {
    epoch: Option<std::time::Instant>,
}

impl SystemClock {
    pub fn new() -> Self {
        Self {
            epoch: Some(std::time::Instant::now()),
        }
    }
}

impl Clock for SystemClock {
    fn now(&self) -> Duration {
        self.epoch.map(|e| e.elapsed()).unwrap_or_default()
    }

    fn sleep(
        &self,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(tokio::time::sleep(duration))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// A manually advanced clock for deterministic tests.
///
/// `sleep` futures complete only when [`TestClock::advance`] moves the clock
/// past their deadline, so a test controls exactly when time passes.
#[derive(Debug, Clone, Default)]
pub struct TestClock {
    inner: Arc<Mutex<TestClockInner>>,
}

#[derive(Debug, Default)]
struct TestClockInner {
    now: Duration,
    waiters: Vec<(Duration, Waker)>,
}

impl TestClock {
    /// Create a clock at the given virtual instant.
    pub fn at(now: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TestClockInner {
                now,
                waiters: Vec::new(),
            })),
        }
    }

    /// Create a clock at time zero.
    pub fn new() -> Self {
        Self::at(Duration::ZERO)
    }

    /// Advance virtual time by `delta`, waking any sleepers it passes.
    pub fn advance(&self, delta: Duration) {
        let mut inner = self.inner.lock().unwrap();
        inner.now += delta;
        let now = inner.now;
        let mut remaining = Vec::new();
        for (deadline, waker) in inner.waiters.drain(..) {
            if deadline <= now {
                waker.wake();
            } else {
                remaining.push((deadline, waker));
            }
        }
        inner.waiters = remaining;
    }

    /// Advance virtual time by the given number of milliseconds.
    pub fn advance_ms(&self, ms: u64) {
        self.advance(Duration::from_millis(ms));
    }
}

impl Clock for TestClock {
    fn now(&self) -> Duration {
        self.inner.lock().unwrap().now
    }

    fn sleep(
        &self,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let inner = self.inner.clone();
        let deadline = self.now() + duration;
        Box::pin(Sleep { inner, deadline })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct Sleep {
    inner: Arc<Mutex<TestClockInner>>,
    deadline: Duration,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut inner = self.inner.lock().unwrap();
        if inner.now >= self.deadline {
            Poll::Ready(())
        } else {
            inner.waiters.push((self.deadline, cx.waker().clone()));
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_reports_advanced_time() {
        let clock = TestClock::new();
        assert_eq!(clock.now(), Duration::ZERO);
        clock.advance_ms(250);
        assert_eq!(clock.now(), Duration::from_millis(250));
    }

    #[test]
    fn sleep_only_resolves_after_advance() {
        let clock = TestClock::new();
        let mut fut = clock.sleep(Duration::from_millis(100));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        assert!(Pin::new(&mut fut).poll(&mut cx).is_pending());

        clock.advance_ms(50);
        assert!(Pin::new(&mut fut).poll(&mut cx).is_pending());

        clock.advance_ms(60);
        assert!(Pin::new(&mut fut).poll(&mut cx).is_ready());
    }

    fn noop_waker() -> Waker {
        struct Noop;
        impl std::task::Wake for Noop {
            fn wake(self: Arc<Self>) {}
        }
        Waker::from(Arc::new(Noop))
    }
}
