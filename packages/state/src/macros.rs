/// Create a named store type with interior state and an initial value.
///
/// # Example
/// ```rust,ignore
/// create_store!(Counter, i32, 0);
/// let store = Counter::new();
/// store.set(42);
/// ```
#[macro_export]
macro_rules! create_store {
    ($name:ident, $state:ty, $initial:expr) => {
        #[derive(Clone)]
        pub struct $name {
            state: std::sync::Arc<std::sync::RwLock<$state>>,
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    state: std::sync::Arc::new(std::sync::RwLock::new(
                        $initial,
                    )),
                }
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $crate::Store for $name {
            type State = $state;
            fn get(&self) -> $state {
                self.state
                    .read()
                    .map(|s| s.clone())
                    .unwrap_or_else(|_| panic!("state lock poisoned"))
            }
            fn set(&self, state: $state) {
                *self
                    .state
                    .write()
                    .unwrap_or_else(|_| panic!("state lock poisoned")) = state;
            }
            fn update<F>(&self, f: F)
            where
                F: FnOnce($state) -> $state + Send + Sync + 'static,
            {
                let current = self.get();
                self.set(f(current));
            }
            fn update_boxed(
                &self,
                f: Box<dyn FnOnce($state) -> $state + Send + Sync>,
            ) {
                let current = self.get();
                self.set(f(current));
            }
        }
    };
}

/// Create a simple store instance with an initial value.
///
/// # Example
/// ```rust,ignore
/// let store = new_store!(i32, 0);
/// ```
#[macro_export]
macro_rules! new_store {
    ($state:ty, $initial:expr) => {
        $crate::SimpleStore::<$state>::new($initial)
    };
}

/// Create a field selector from a closure.
///
/// # Example
/// ```rust,ignore
/// let selector = selector!(|state: &MyState| state.field.clone());
/// ```
#[macro_export]
macro_rules! selector {
    ($closure:expr) => {
        $crate::FieldSelector::new($closure)
    };
}
