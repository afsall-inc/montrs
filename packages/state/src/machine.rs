use crate::error::{StateError, StateResult};
use std::fmt;

pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + fmt::Debug + 'static;
    type Event: Clone + Send + Sync + fmt::Debug + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + fmt::Debug + 'static;
    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}

pub trait MachineState {
    type Context: Send + Sync + 'static;
    fn value(&self) -> &str;
    fn context(&self) -> &Self::Context;
    fn matches(&self, pattern: &str) -> bool;
    fn can_transition_to(&self, _target: &str) -> bool { true }
    fn construct(value: String, context: Self::Context) -> Self;
}

pub trait Action<C, E>: Send + Sync {
    fn execute(&self, context: &mut C, event: &E);
    fn name(&self) -> &str;
    fn description(&self) -> String { self.name().to_string() }
}

pub trait Guard<C, E>: Send + Sync {
    fn check(&self, context: &C, event: &E) -> bool;
    fn name(&self) -> &str;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MachineStateImpl<C> {
    value: String,
    context: C,
}

impl<C> MachineStateImpl<C> {
    pub fn new(value: impl Into<String>, context: C) -> Self {
        Self { value: value.into(), context }
    }
}

impl<C: Clone + Send + Sync + 'static> MachineState for MachineStateImpl<C> {
    type Context = C;
    fn value(&self) -> &str { &self.value }
    fn context(&self) -> &C { &self.context }
    fn matches(&self, pattern: &str) -> bool {
        pattern == "*" || self.value == pattern || self.value.starts_with(pattern)
    }
    fn construct(value: String, context: Self::Context) -> Self {
        Self::new(value, context)
    }
}

/// A single transition definition with guards and actions.
pub struct Transition<C, E> {
    pub from: String,
    pub event: E,
    pub to: String,
    pub guards: Vec<Box<dyn Guard<C, E>>>,
    pub actions: Vec<Box<dyn Action<C, E>>>,
}

impl<C, E> Transition<C, E> {
    pub fn new(from: impl Into<String>, event: E, to: impl Into<String>) -> Self {
        Self { from: from.into(), event, to: to.into(), guards: Vec::new(), actions: Vec::new() }
    }
}

/// A state node with outgoing transitions and lifecycle actions.
pub struct StateNode<C, E> {
    pub name: String,
    pub transitions: Vec<Transition<C, E>>,
    pub entry_actions: Vec<Box<dyn Action<C, E>>>,
    pub exit_actions: Vec<Box<dyn Action<C, E>>>,
}

/// XState-inspired machine executing typed state/event transitions.
pub struct Machine<C, E, S> {
    current: S,
    pub(crate) states: Vec<StateNode<C, E>>,
    context: C,
}

impl<C, E, S> fmt::Debug for Machine<C, E, S>
where C: fmt::Debug, S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine").field("current", &self.current).field("context", &self.context).finish()
    }
}

impl<C, E, S> Machine<C, E, S>
where C: Clone + PartialEq + Send + Sync + 'static, E: Clone + PartialEq + fmt::Debug + 'static, S: MachineState<Context = C> + Clone + 'static {
    pub fn new(initial: S, context: C) -> Self { Self { current: initial, states: Vec::new(), context } }
    pub fn state(&self) -> &S { &self.current }
    pub fn context(&self) -> &C { &self.context }
    pub fn context_mut(&mut self) -> &mut C { &mut self.context }
    pub fn send(&mut self, event: &E) -> StateResult<&S> {
        let current_value = self.current.value().to_string();
        let transition = self.states.iter()
            .flat_map(|s| s.transitions.iter())
            .find(|t| t.from == current_value && t.event == *event)
            .ok_or_else(|| StateError::InvalidTransition { from: current_value, event: format!("{event:?}") })?;
        for guard in &transition.guards { if !guard.check(&self.context, event) { return Err(StateError::GuardRejected); } }
        for action in &transition.actions { action.execute(&mut self.context, event); }
        self.current = S::construct(transition.to.clone(), self.context.clone());
        Ok(&self.current)
    }
}

/// Fluent builder for machines.
#[allow(clippy::new_without_default)]
pub struct MachineBuilder<C, E> {
    states: Vec<StateNode<C, E>>,
    initial: String,
    context: Option<C>,
}

impl<C, E> Default for MachineBuilder<C, E>
where C: Clone + Default + PartialEq + Send + Sync + 'static, E: Clone + PartialEq + fmt::Debug + 'static {
    fn default() -> Self { Self::new() }
}

impl<C, E> MachineBuilder<C, E>
where C: Clone + Default + PartialEq + Send + Sync + 'static, E: Clone + PartialEq + fmt::Debug + 'static {
    pub fn new() -> Self { Self { states: Vec::new(), initial: String::new(), context: None } }
    pub fn initial(mut self, state: impl Into<String>) -> Self { self.initial = state.into(); self }
    pub fn context(mut self, context: C) -> Self { self.context = Some(context); self }
    pub fn transition(mut self, from: impl Into<String>, event: E, to: impl Into<String>) -> Self {
        let from = from.into(); let to = to.into();
        if let Some(node) = self.states.iter_mut().find(|n| n.name == from) {
            node.transitions.push(Transition::new(from.clone(), event, to));
        } else {
            let mut node = StateNode { name: from.clone(), transitions: Vec::new(), entry_actions: Vec::new(), exit_actions: Vec::new() };
            node.transitions.push(Transition::new(from, event, to));
            self.states.push(node);
        }
        self
    }
    pub fn build(self) -> StateResult<Machine<C, E, MachineStateImpl<C>>> {
        let context = self.context.unwrap_or_default();
        let initial = MachineStateImpl::new(self.initial, context.clone());
        let mut machine = Machine::new(initial, context);
        machine.states = self.states;
        Ok(machine)
    }
}

// ============================================================================
// Action implementations
// ============================================================================

pub struct FunctionAction<C, E, F> { f: F, name: String, _phantom: std::marker::PhantomData<fn(C, E)> }
impl<C, E, F> FunctionAction<C, E, F> where F: Fn(&mut C, &E) + Send + Sync + 'static {
    pub fn new(name: impl Into<String>, f: F) -> Self { Self { f, name: name.into(), _phantom: std::marker::PhantomData } }
}
impl<C, E, F> Action<C, E> for FunctionAction<C, E, F> where F: Fn(&mut C, &E) + Send + Sync + 'static {
    fn execute(&self, context: &mut C, event: &E) { (self.f)(context, event); }
    fn name(&self) -> &str { &self.name }
}

pub struct LogAction { name: String }
impl LogAction { pub fn new(name: impl Into<String>) -> Self { Self { name: name.into() } } }
impl<C, E> Action<C, E> for LogAction where C: fmt::Debug, E: fmt::Debug {
    fn execute(&self, _context: &mut C, event: &E) { log::info!(target: "montrs_state", "action: {} event: {:?}", self.name, event); }
    fn name(&self) -> &str { &self.name }
}

pub struct AssignAction<C, E, T, F> { #[allow(dead_code)] f: F, name: String, _phantom: std::marker::PhantomData<fn(C, E, T)> }
impl<C, E, T, F> AssignAction<C, E, T, F> where F: Fn(&C, &E) -> T + Send + Sync + 'static, C: Send + Sync, E: Send + Sync, T: Send + Sync + 'static {
    pub fn new(name: impl Into<String>, f: F) -> Self { Self { f, name: name.into(), _phantom: std::marker::PhantomData } }
}
impl<C, E, T, F> Action<C, E> for AssignAction<C, E, T, F> where F: Fn(&C, &E) -> T + Send + Sync + 'static, C: Send + Sync, E: Send + Sync, T: Send + Sync + 'static {
    fn execute(&self, _context: &mut C, _event: &E) { }
    fn name(&self) -> &str { &self.name }
}

// ============================================================================
// Guard implementations
// ============================================================================

pub struct FunctionGuard<C, E, F> { f: F, name: String, _phantom: std::marker::PhantomData<fn(C, E)> }
impl<C, E, F> FunctionGuard<C, E, F> where F: Fn(&C, &E) -> bool + Send + Sync + 'static {
    pub fn new(name: impl Into<String>, f: F) -> Self { Self { f, name: name.into(), _phantom: std::marker::PhantomData } }
}
impl<C, E, F> Guard<C, E> for FunctionGuard<C, E, F> where F: Fn(&C, &E) -> bool + Send + Sync + 'static {
    fn check(&self, context: &C, event: &E) -> bool { (self.f)(context, event) }
    fn name(&self) -> &str { &self.name }
}

pub struct AndGuard<C, E> { guards: Vec<Box<dyn Guard<C, E>>>, name: String }
impl<C, E> AndGuard<C, E> {
    pub fn new(name: impl Into<String>, guards: Vec<Box<dyn Guard<C, E>>>) -> Self { Self { guards, name: name.into() } }
}
impl<C, E> Guard<C, E> for AndGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool { self.guards.iter().all(|g| g.check(context, event)) }
    fn name(&self) -> &str { &self.name }
}

pub struct OrGuard<C, E> { guards: Vec<Box<dyn Guard<C, E>>>, name: String }
impl<C, E> OrGuard<C, E> {
    pub fn new(name: impl Into<String>, guards: Vec<Box<dyn Guard<C, E>>>) -> Self { Self { guards, name: name.into() } }
}
impl<C, E> Guard<C, E> for OrGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool { self.guards.iter().any(|g| g.check(context, event)) }
    fn name(&self) -> &str { &self.name }
}

pub struct NotGuard<C, E> { inner: Box<dyn Guard<C, E>>, name: String }
impl<C, E> NotGuard<C, E> {
    pub fn new(name: impl Into<String>, inner: Box<dyn Guard<C, E>>) -> Self { Self { inner, name: name.into() } }
}
impl<C, E> Guard<C, E> for NotGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool { !self.inner.check(context, event) }
    fn name(&self) -> &str { &self.name }
}

// ============================================================================
// History
// ============================================================================

#[derive(Clone, Debug)]
pub struct HistoryEntry<C> {
    pub state: String,
    pub context: C,
    pub timestamp: std::time::Instant,
}

#[derive(Clone, Debug)]
pub struct HistoryTracker<C> {
    entries: Vec<HistoryEntry<C>>,
    limit: usize,
}

impl<C> HistoryTracker<C> {
    pub fn new(limit: usize) -> Self { Self { entries: Vec::new(), limit } }
    pub fn push(&mut self, entry: HistoryEntry<C>) { self.entries.push(entry); while self.entries.len() > self.limit { self.entries.remove(0); } }
    pub fn entries(&self) -> &[HistoryEntry<C>] { &self.entries }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}

#[derive(Clone, Debug)]
pub struct MachineHistory<C> { tracker: HistoryTracker<C> }

impl<C> MachineHistory<C> {
    pub fn new(limit: usize) -> Self { Self { tracker: HistoryTracker::new(limit) } }
    pub fn push(&mut self, state: impl Into<String>, context: C) {
        self.tracker.push(HistoryEntry { state: state.into(), context, timestamp: std::time::Instant::now() });
    }
    pub fn entries(&self) -> &[HistoryEntry<C>] { self.tracker.entries() }
    pub fn len(&self) -> usize { self.tracker.len() }
    pub fn is_empty(&self) -> bool { self.tracker.is_empty() }
}

pub type HistoryMachine<C, E> = Machine<C, E, MachineStateImpl<C>>;

// ============================================================================
// Event types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringEvent(pub String);
impl StringEvent { pub fn new(value: impl Into<String>) -> Self { Self(value.into()) } }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataEvent<T> { pub event_type: String, pub data: T }
impl<T> DataEvent<T> {
    pub fn new(event_type: impl Into<String>, data: T) -> Self { Self { event_type: event_type.into(), data } }
}

// ============================================================================
// MachineError
// ============================================================================

#[derive(Clone, Debug)]
pub struct MachineError { pub message: String }
impl MachineError {
    pub fn new(message: impl Into<String>) -> Self { Self { message: message.into() } }
    pub fn from_state_error(error: StateError) -> Self { Self { message: error.to_string() } }
}
impl fmt::Display for MachineError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.message) } }
impl std::error::Error for MachineError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct CounterEvent(String);

    #[test]
    fn machine_builder_works() {
        let mut machine = MachineBuilder::<i32, CounterEvent>::new()
            .initial("idle")
            .transition("idle", CounterEvent("start".into()), "running")
            .transition("running", CounterEvent("stop".into()), "idle")
            .build().unwrap();
        assert_eq!(machine.state().value(), "idle");
        machine.send(&CounterEvent("start".into())).unwrap();
        assert_eq!(machine.state().value(), "running");
    }

    #[test]
    fn action_executes() {
        let mut context = 0_i32;
        let action = FunctionAction::new("increment", |c: &mut i32, _e: &StringEvent| *c += 1);
        action.execute(&mut context, &StringEvent("tick".into()));
        assert_eq!(context, 1);
    }

    #[test]
    fn guard_rejects() {
        let guard = FunctionGuard::new("always_false", |_: &i32, _: &StringEvent| false);
        assert!(!guard.check(&0, &StringEvent("test".into())));
    }
}
