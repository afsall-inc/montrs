//! Production-grade unit testing utilities for MontRS.
//!
//! This plate provides a rich set of testing primitives inspired by modern
//! testing frameworks like Jest. It focuses on Developer Experience (DevX)
//! by providing fluent assertions, mocking/spying utilities, and benchmarking tools.
//!
//! # Features
//!
//! - **Fluent Assertions**: `expect(value).to_equal(other)` style assertions.
//! - **Spies**: Track function calls and interactions.
//! - **Benchmarking**: Simple performance measurement tools.
//! - **Table-Driven Tests**: Macros for parameterized testing.

use std::{
    fmt::Debug,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

// =============================================================================
//  Fluent Assertions
// =============================================================================

/// Creates a new expectation for a value.
///
/// # Example
///
/// ```rust
/// use montrs_test::unit::expect;
///
/// expect(1 + 1).to_equal(2);
/// expect(true).to_be_true();
/// ```
pub fn expect<T>(value: T) -> Expectation<T> {
    Expectation::new(value)
}

/// A wrapper around a value to perform assertions.
pub struct Expectation<T> {
    value: T,
    negated: bool,
}

impl<T> Expectation<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            negated: false,
        }
    }

    /// Negates the next assertion.
    ///
    /// # Example
    ///
    /// ```rust
    /// use montrs_test::unit::expect;
    /// expect(1).not().to_equal(2);
    /// ```
    pub fn not(mut self) -> Self {
        self.negated = !self.negated;
        self
    }
}

impl<T: Debug + PartialEq> Expectation<T> {
    /// Asserts that the value equals the expected value.
    pub fn to_equal(&self, other: T) {
        if self.negated {
            if self.value == other {
                panic!("Expected value NOT to equal {:?}, but it did.", other);
            }
        } else if self.value != other {
            panic!(
                "Expected value to equal {:?}, but found {:?}.",
                other, self.value
            );
        }
    }

    /// Asserts that the value does not equal the expected value.
    /// Alias for `.not().to_equal()`.
    pub fn to_not_equal(&self, other: T) {
        if self.negated {
            // double negation -> assert equal
            if self.value != other {
                panic!(
                    "Expected value to equal {:?}, but found {:?}.",
                    other, self.value
                );
            }
        } else if self.value == other {
            panic!("Expected value NOT to equal {:?}, but it did.", other);
        }
    }
}

impl Expectation<bool> {
    pub fn to_be_true(&self) {
        if self.negated {
            assert!(!self.value, "Expected false, but found true");
        } else {
            assert!(self.value, "Expected true, but found false");
        }
    }

    pub fn to_be_false(&self) {
        if self.negated {
            assert!(self.value, "Expected true, but found false");
        } else {
            assert!(!self.value, "Expected false, but found true");
        }
    }
}

impl<T: Debug, E: Debug> Expectation<Result<T, E>> {
    pub fn to_be_ok(&self) {
        if self.negated {
            if self.value.is_ok() {
                panic!(
                    "Expected Err, but found Ok({:?})",
                    self.value.as_ref().unwrap()
                );
            }
        } else if self.value.is_err() {
            panic!(
                "Expected Ok, but found Err({:?})",
                self.value.as_ref().err().unwrap()
            );
        }
    }

    pub fn to_be_err(&self) {
        if self.negated {
            if self.value.is_err() {
                panic!(
                    "Expected Ok, but found Err({:?})",
                    self.value.as_ref().err().unwrap()
                );
            }
        } else if self.value.is_ok() {
            panic!(
                "Expected Err, but found Ok({:?})",
                self.value.as_ref().unwrap()
            );
        }
    }
}

impl<T: Debug> Expectation<Option<T>> {
    pub fn to_be_some(&self) {
        if self.negated {
            if self.value.is_some() {
                panic!(
                    "Expected None, but found Some({:?})",
                    self.value.as_ref().unwrap()
                );
            }
        } else if self.value.is_none() {
            panic!("Expected Some, but found None");
        }
    }

    pub fn to_be_none(&self) {
        if self.negated {
            if self.value.is_none() {
                panic!("Expected Some, but found None");
            }
        } else if self.value.is_some() {
            panic!(
                "Expected None, but found Some({:?})",
                self.value.as_ref().unwrap()
            );
        }
    }
}

impl<T: Debug + PartialOrd> Expectation<T> {
    pub fn to_be_greater_than(&self, other: T) {
        if self.negated {
            if self.value > other {
                panic!(
                    "Expected {:?} NOT to be greater than {:?}",
                    self.value, other
                );
            }
        } else if self.value <= other {
            panic!("Expected {:?} to be greater than {:?}", self.value, other);
        }
    }

    pub fn to_be_less_than(&self, other: T) {
        if self.negated {
            if self.value < other {
                panic!(
                    "Expected {:?} NOT to be less than {:?}",
                    self.value, other
                );
            }
        } else if self.value >= other {
            panic!("Expected {:?} to be less than {:?}", self.value, other);
        }
    }
}

impl<T: Debug + PartialEq> Expectation<Vec<T>> {
    pub fn to_contain(&self, item: &T) {
        let contains = self.value.contains(item);
        if self.negated {
            if contains {
                panic!("Expected list NOT to contain {:?}, but it did.", item);
            }
        } else if !contains {
            panic!("Expected list to contain {:?}, but it did not.", item);
        }
    }

    pub fn to_have_length(&self, length: usize) {
        if self.negated {
            if self.value.len() == length {
                panic!(
                    "Expected list NOT to have length {}, but it did.",
                    length
                );
            }
        } else if self.value.len() != length {
            panic!(
                "Expected list to have length {}, but found {}.",
                length,
                self.value.len()
            );
        }
    }
}

// =============================================================================
//  Spies & Mocks
// =============================================================================

/// A utility to track function calls.
///
/// # Example
///
/// ```rust
/// use montrs_test::unit::Spy;
///
/// let spy = Spy::new();
/// spy.record();
/// assert!(spy.called());
/// ```
#[derive(Clone, Default, Debug)]
pub struct Spy {
    calls: Arc<AtomicUsize>,
}

impl Spy {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a call to the spy.
    pub fn record(&self) {
        self.calls.fetch_add(1, Ordering::SeqCst);
    }

    /// Returns true if the spy has been called at least once.
    pub fn called(&self) -> bool {
        self.calls.load(Ordering::SeqCst) > 0
    }

    /// Returns the number of times the spy was called.
    pub fn call_count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }

    /// Resets the spy.
    pub fn reset(&self) {
        self.calls.store(0, Ordering::SeqCst);
    }
}

/// A generic mock that can store arguments and provide return values.
#[derive(Clone, Debug)]
pub struct Mock<Args, Ret> {
    pub calls: Arc<Mutex<Vec<Args>>>,
    pub return_value: Arc<Mutex<Option<Ret>>>,
}

impl<Args: Clone + Send, Ret: Clone + Send> Default for Mock<Args, Ret> {
    fn default() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            return_value: Arc::new(Mutex::new(None)),
        }
    }
}

impl<Args: Clone + Send, Ret: Clone + Send> Mock<Args, Ret> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mock_return(&self, value: Ret) {
        let mut ret = self.return_value.lock().unwrap();
        *ret = Some(value);
    }

    pub fn call(&self, args: Args) -> Option<Ret> {
        self.calls.lock().unwrap().push(args);
        self.return_value.lock().unwrap().clone()
    }

    pub fn called_with(&self, args: &Args) -> bool
    where
        Args: PartialEq,
    {
        self.calls.lock().unwrap().contains(args)
    }

    pub fn call_count(&self) -> usize {
        self.calls.lock().unwrap().len()
    }
}

// =============================================================================
//  Benchmarking
// =============================================================================

pub use montrs_bench::{BenchConfig, Benchmark};

// Re-export old simple bench for backward compatibility if needed,
// or deprecate it. For now, we assume the user wants the new power.
// We can wrap montrs-bench here if we want a simpler API surface.

/// Runs the provided async function multiple times and prints timing statistics.
///
/// This uses the `montrs-bench` engine under the hood.
pub async fn simple_bench<F, Fut>(name: &str, iterations: u32, func: F)
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    use montrs_bench::{BenchConfig, BenchRunner, SimpleBench};

    let mut config = BenchConfig::default();
    config.iterations = iterations;

    let mut runner = BenchRunner::with_config(config);

    let bench = SimpleBench::new(name, move || {
        let f = func();
        async move {
            f.await;
            Ok(())
        }
    });

    runner.add(bench);

    if let Err(e) = runner.run().await {
        eprintln!("Benchmark failed: {}", e);
    }
}

// =============================================================================
//  Macros
// =============================================================================

/// A macro for defining table-driven tests.
///
/// # Example
///
/// ```rust
/// use montrs_test::table_test;
///
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// table_test! {
///     name: test_add,
///     func: add,
///     cases: [
///         (1, 1) => 2,
///         (2, 2) => 4,
///         (10, -5) => 5,
///     ]
/// }
/// ```
#[macro_export]
macro_rules! table_test {
    (
        name: $test_name:ident,
        func: $func:ident,
        cases: [
            $( ($($arg:expr),*) => $expected:expr ),* $(,)?
        ]
    ) => {
        #[test]
        fn $test_name() {
            $(
                let result = $func($($arg),*);
                assert_eq!(result, $expected, "Failed on case: {:?}", ($($arg),*));
            )*
        }
    };
}
