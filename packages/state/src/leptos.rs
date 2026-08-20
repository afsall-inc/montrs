use crate::{
    error::StateResult,
    machine::{Machine, MachineHistory, MachineState, StateMachine},
    store::{SimpleStore, Store, StoreContext, StoreSlice},
    time_travel::TimeTravel,
};
use leptos::prelude::*;

pub fn provide_store<T>(store: StoreContext<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    provide_context(store);
}

pub fn use_store_value<T>() -> T
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    expect_context::<StoreContext<T>>().get()
}

pub struct StoreActions<T> {
    store: StoreContext<T>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> StoreActions<T> {
    pub fn set(&self, state: T) {
        self.store.set(state);
    }
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T + Send + Sync + 'static,
    {
        self.store.update(f);
    }
    pub fn reset(&self, initial: T) {
        self.store.set(initial);
    }
}

pub fn use_store<T>() -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let store = expect_context::<StoreContext<T>>();
    let (signal, set_signal) = signal(store.get());
    Effect::new(move |_| {
        set_signal.set(store.get());
    });
    (signal, set_signal)
}

pub fn use_store_with_actions<T>() -> (ReadSignal<T>, StoreActions<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let store = expect_context::<StoreContext<T>>();
    let (signal, set_signal) = signal(store.get());
    let actions = StoreActions {
        store: store.clone(),
    };
    Effect::new(move |_| {
        set_signal.set(store.get());
    });
    (signal, actions)
}

pub fn use_store_slice<S, Slice>(
    store: StoreContext<S>,
    selector: Slice,
) -> Memo<Slice::Output>
where
    S: Clone + PartialEq + Send + Sync + 'static,
    Slice: StoreSlice<SimpleStore<S>> + 'static,
    Slice::Output: Clone + PartialEq + 'static,
{
    let selector = std::sync::Arc::new(selector);
    let store_clone = store.clone();
    Memo::new(move |_| selector.select(&store_clone.get()))
}

pub fn use_store_history<T>() -> (ReadSignal<T>, TimeTravel<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let store = expect_context::<StoreContext<T>>();
    let (signal, set_signal) = signal(store.get());
    let mut time_travel = TimeTravel::new(store.get(), 32);
    Effect::new(move |_| {
        let current = store.get();
        set_signal.set(current.clone());
    });
    (signal, time_travel)
}

pub struct MachineHandle<M: StateMachine> {
    pub state: ReadSignal<M::State>,
    pub send: Callback<M::Event>,
    pub context: M::Context,
}

pub fn use_machine<M: StateMachine + 'static>() -> MachineHandle<M>
where
    M::State: Clone + 'static,
    M::Event: Clone + 'static,
    M::Context: Clone + 'static,
{
    let initial = M::initial();
    let (state, set_state) = signal(initial);
    let context = state.get().context().clone();
    let send = Callback::new(move |event: M::Event| {
        let current = state.get();
        let next = M::transition(&current, event);
        set_state.set(next);
    });
    MachineHandle {
        state,
        send,
        context,
    }
}

pub fn use_machine_with_instance<M: StateMachine + 'static>(
    machine: M,
) -> (ReadSignal<M::State>, Callback<M::Event>)
where
    M::State: Clone + 'static,
    M::Event: Clone + 'static,
{
    let initial = M::initial();
    let (state, set_state) = signal(initial);
    let send = Callback::new(move |event: M::Event| {
        let current = state.get();
        let next = M::transition(&current, event);
        set_state.set(next);
    });
    (state, send)
}

pub fn use_machine_history<M: StateMachine + 'static>()
-> MachineHistory<M::Context>
where
    M::Context: Clone + 'static,
{
    MachineHistory::new(32)
}

pub fn provide_store_simple<T>(store: SimpleStore<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    provide_context(StoreContext::new(store));
}
