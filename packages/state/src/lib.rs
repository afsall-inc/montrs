//! Deterministic state primitives for MontRS.
//!
//! The core deliberately does not depend on Leptos. Applications can use the
//! same store and machine models from Web, TUI, desktop, and server code.

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum StateError {
    #[error("state lock is poisoned")]
    LockPoisoned,
    #[error("invalid state transition from {from} on event {event}")]
    InvalidTransition { from: String, event: String },
}

pub type StateResult<T> = Result<T, StateError>;

pub trait Reducer<S, E>: Send + Sync {
    fn reduce(&self, state: &S, event: &E) -> StateResult<S>;
}

impl<S, E, F> Reducer<S, E> for F
where
    F: Fn(&S, &E) -> StateResult<S> + Send + Sync,
{
    fn reduce(&self, state: &S, event: &E) -> StateResult<S> {
        self(state, event)
    }
}

#[derive(Clone)]
pub struct Store<S, E> {
    state: Arc<RwLock<S>>,
    reducer: Arc<dyn Reducer<S, E>>,
    history: Arc<RwLock<VecDeque<S>>>,
    history_limit: usize,
}

impl<S, E> fmt::Debug for Store<S, E>
where
    S: Clone + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Store").field("state", &self.state()).finish()
    }
}

impl<S, E> Store<S, E>
where
    S: Clone,
{
    pub fn new(initial: S, reducer: impl Reducer<S, E> + 'static) -> Self {
        Self::with_history_limit(initial, reducer, 32)
    }

    pub fn with_history_limit(
        initial: S,
        reducer: impl Reducer<S, E> + 'static,
        history_limit: usize,
    ) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
            reducer: Arc::new(reducer),
            history: Arc::new(RwLock::new(VecDeque::new())),
            history_limit,
        }
    }

    pub fn state(&self) -> StateResult<S> {
        self.state
            .read()
            .map(|state| state.clone())
            .map_err(|_| StateError::LockPoisoned)
    }

    pub fn select<T>(&self, selector: impl FnOnce(&S) -> T) -> StateResult<T> {
        self.state
            .read()
            .map(|state| selector(&state))
            .map_err(|_| StateError::LockPoisoned)
    }

    pub fn dispatch(&self, event: &E) -> StateResult<S> {
        let current = self.state()?;
        let next = self.reducer.reduce(&current, event)?;
        if self.history_limit > 0 {
            let mut history = self.history.write().map_err(|_| StateError::LockPoisoned)?;
            history.push_back(current);
            while history.len() > self.history_limit {
                history.pop_front();
            }
        }
        *self.state.write().map_err(|_| StateError::LockPoisoned)? = next.clone();
        Ok(next)
    }

    pub fn undo(&self) -> StateResult<Option<S>> {
        let previous = self
            .history
            .write()
            .map_err(|_| StateError::LockPoisoned)?
            .pop_back();
        if let Some(previous) = previous {
            *self.state.write().map_err(|_| StateError::LockPoisoned)? = previous.clone();
            Ok(Some(previous))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition<S, E> {
    pub from: S,
    pub event: E,
    pub to: S,
}

#[derive(Debug, Clone)]
pub struct StateMachine<S, E> {
    state: S,
    transitions: Vec<Transition<S, E>>,
}

impl<S, E> StateMachine<S, E>
where
    S: Clone + PartialEq + fmt::Display,
    E: Clone + PartialEq + fmt::Display,
{
    pub fn new(initial: S) -> Self {
        Self {
            state: initial,
            transitions: Vec::new(),
        }
    }

    pub fn transition(mut self, from: S, event: E, to: S) -> Self {
        self.transitions.push(Transition { from, event, to });
        self
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn send(&mut self, event: &E) -> StateResult<&S> {
        let transition = self
            .transitions
            .iter()
            .find(|transition| transition.from == self.state && transition.event == *event)
            .cloned()
            .ok_or_else(|| StateError::InvalidTransition {
                from: self.state.to_string(),
                event: event.to_string(),
            })?;
        self.state = transition.to;
        Ok(&self.state)
    }
}

#[cfg(feature = "leptos")]
pub mod leptos {
    use super::Store;
    use ::leptos::prelude::*;

    pub fn provide_store<S, E>(store: Store<S, E>)
    where
        S: Clone + Send + Sync + 'static,
        E: Send + Sync + 'static,
    {
        provide_context(store);
    }

    pub fn use_store<S, E>() -> Store<S, E>
    where
        S: Clone + Send + Sync + 'static,
        E: Send + Sync + 'static,
    {
        expect_context::<Store<S, E>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Event {
        Increment,
        Reset,
    }

    impl fmt::Display for Event {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }

    #[test]
    fn store_dispatch_selects_and_undoes() {
        let store = Store::new(0_i32, |state: &i32, event: &Event| match event {
            Event::Increment => Ok(state + 1),
            Event::Reset => Ok(0),
        });
        assert_eq!(store.dispatch(&Event::Increment), Ok(1));
        assert_eq!(store.dispatch(&Event::Reset), Ok(0));
        assert_eq!(store.select(|state| *state), Ok(0));
        assert_eq!(store.undo(), Ok(Some(1)));
    }

    #[test]
    fn machine_transitions_are_explicit() {
        let machine = StateMachine::new("idle")
            .transition("idle", "start", "running")
            .transition("running", "stop", "idle");
        let mut machine = machine;
        assert_eq!(machine.send(&"start"), Ok(&"running"));
        assert!(machine.send(&"stop").is_ok());
    }
}
