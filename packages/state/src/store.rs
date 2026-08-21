use std::{
    fmt,
    sync::{Arc, RwLock},
};

pub trait Store: Send + Sync + 'static {
    type State: Clone + PartialEq + Send + Sync + 'static;
    fn get(&self) -> Self::State;
    fn set(&self, state: Self::State);
    fn update_boxed(
        &self,
        f: Box<dyn FnOnce(Self::State) -> Self::State + Send + Sync>,
    );
    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State + Send + Sync + 'static;
}

impl<T: Store> Store for Arc<T> {
    type State = T::State;
    fn get(&self) -> Self::State {
        (**self).get()
    }
    fn set(&self, state: Self::State) {
        (**self).set(state)
    }
    fn update_boxed(
        &self,
        f: Box<dyn FnOnce(Self::State) -> Self::State + Send + Sync>,
    ) {
        (**self).update_boxed(f)
    }
    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State + Send + Sync + 'static,
    {
        (**self).update(f)
    }
}

pub struct SimpleStore<T: Clone + PartialEq + Send + Sync + 'static> {
    pub(crate) state: Arc<RwLock<T>>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> SimpleStore<T> {
    pub fn new(initial: T) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
        }
    }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> Store for SimpleStore<T> {
    type State = T;
    fn get(&self) -> T {
        self.state
            .read()
            .map(|s| s.clone())
            .unwrap_or_else(|_| panic!("lock poisoned"))
    }
    fn set(&self, state: T) {
        *self
            .state
            .write()
            .unwrap_or_else(|_| panic!("lock poisoned")) = state;
    }
    fn update_boxed(&self, f: Box<dyn FnOnce(T) -> T + Send + Sync>) {
        let current = self.get();
        self.set(f(current));
    }
    fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T + Send + Sync + 'static,
    {
        self.update_boxed(Box::new(f))
    }
}

impl<T> fmt::Debug for SimpleStore<T>
where
    T: Clone + PartialEq + Send + Sync + 'static + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleStore")
            .field("state", &self.get())
            .finish()
    }
}

#[derive(Clone)]
pub struct StoreContext<T: Clone + PartialEq + Send + Sync + 'static> {
    inner: Arc<RwLock<T>>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> StoreContext<T> {
    pub fn new(store: SimpleStore<T>) -> Self {
        Self {
            inner: store.state.clone(),
        }
    }
    pub fn get(&self) -> T {
        self.inner
            .read()
            .map(|s| s.clone())
            .unwrap_or_else(|_| panic!("lock poisoned"))
    }
    pub fn set(&self, state: T) {
        *self
            .inner
            .write()
            .unwrap_or_else(|_| panic!("lock poisoned")) = state;
    }
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T + Send + Sync + 'static,
    {
        let current = self.get();
        self.set(f(current));
    }
}

pub trait StoreSlice<T: Store> {
    type Output: Clone + PartialEq + 'static;
    fn select(&self, state: &T::State) -> Self::Output;
}

pub struct FieldSelector<T, O> {
    selector: Arc<dyn Fn(&T) -> O + Send + Sync>,
}

impl<T, O> FieldSelector<T, O> {
    pub fn new(selector: impl Fn(&T) -> O + Send + Sync + 'static) -> Self {
        Self {
            selector: Arc::new(selector),
        }
    }
}

impl<T, O> StoreSlice<SimpleStore<T>> for FieldSelector<T, O>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    O: Clone + PartialEq + 'static,
{
    type Output = O;
    fn select(&self, state: &T) -> O {
        (self.selector)(state)
    }
}

pub trait Middleware<T: Store>: Send + Sync {
    fn on_get(&self, state: &T::State) -> T::State {
        state.clone()
    }
    fn on_set(&self, _old: &T::State, new: &T::State) -> T::State {
        new.clone()
    }
}

pub struct MiddlewareChain<T: Store> {
    middlewares: Vec<Box<dyn Middleware<T>>>,
}

impl<T: Store> Default for MiddlewareChain<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Store> MiddlewareChain<T> {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }
    pub fn add(&mut self, middleware: impl Middleware<T> + 'static) {
        self.middlewares.push(Box::new(middleware));
    }
    pub fn apply_get(&self, state: &T::State) -> T::State {
        let mut state = state.clone();
        for m in &self.middlewares {
            state = m.on_get(&state);
        }
        state
    }
    pub fn apply_set(&self, old: &T::State, new: &T::State) -> T::State {
        let mut state = new.clone();
        for m in &self.middlewares {
            state = m.on_set(old, &state);
        }
        state
    }
}

pub struct LoggerMiddleware;

impl<T: Store> Middleware<T> for LoggerMiddleware
where
    T::State: fmt::Debug,
{
    fn on_get(&self, state: &T::State) -> T::State {
        log::debug!(target: "montrs_state", "get: {:?}", state);
        state.clone()
    }
    fn on_set(&self, old: &T::State, new: &T::State) -> T::State {
        log::info!(target: "montrs_state", "set: {:?} -> {:?}", old, new);
        new.clone()
    }
}

pub struct ValidationMiddleware<T: Store, F> {
    validator: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Store, F> ValidationMiddleware<T, F>
where
    F: Fn(&T::State) -> bool + Send + Sync,
{
    pub fn new(validator: F) -> Self {
        Self {
            validator,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Store, F> Middleware<T> for ValidationMiddleware<T, F>
where
    F: Fn(&T::State) -> bool + Send + Sync,
{
    fn on_set(&self, _old: &T::State, new: &T::State) -> T::State {
        if (self.validator)(new) {
            new.clone()
        } else {
            _old.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone, PartialEq, Debug)]
    struct Counter(i32);
    #[test]
    fn simple_store_get_set_update() {
        let store = SimpleStore::new(Counter(0));
        assert_eq!(store.get().0, 0);
        store.set(Counter(42));
        assert_eq!(store.get().0, 42);
        store.update(|mut s| {
            s.0 += 1;
            s
        });
        assert_eq!(store.get().0, 43);
    }
    #[test]
    fn field_selector_works() {
        let store = SimpleStore::new(Counter(42));
        let selector = FieldSelector::new(|s: &Counter| s.0);
        assert_eq!(selector.select(&store.get()), 42);
    }
}
