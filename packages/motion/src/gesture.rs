use leptos::prelude::*;
use std::cell::RefCell;

/// Reactive hover state for an element.
/// Returns `(on_mouse_enter, on_mouse_leave, is_hovered)`.
pub fn use_hover() -> (
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    ReadSignal<bool>,
) {
    let (hovered, set_hovered) = signal(false);
    let on_enter = move |_: leptos::ev::MouseEvent| set_hovered.set(true);
    let on_leave = move |_: leptos::ev::MouseEvent| set_hovered.set(false);
    (on_enter, on_leave, hovered)
}

/// Reactive press/tap state.
pub fn use_press() -> (
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    ReadSignal<bool>,
) {
    let (pressed, set_pressed) = signal(false);
    let on_down = move |_: leptos::ev::MouseEvent| set_pressed.set(true);
    let on_up = move |_: leptos::ev::MouseEvent| set_pressed.set(false);
    (on_down, on_up, pressed)
}

/// Pan/drag tracking with real delta calculation.
/// Returns `(on_mousedown, on_mousemove, on_mouseup, delta_signal, is_dragging)`.
#[allow(clippy::type_complexity)]
pub fn use_pan() -> (
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    impl Fn(leptos::ev::MouseEvent) + Clone,
    ReadSignal<(f64, f64)>,
    ReadSignal<bool>,
) {
    let (delta, set_delta) = signal((0.0f64, 0.0f64));
    let (dragging, set_dragging) = signal(false);
    let start_pos = std::rc::Rc::new(RefCell::new((0.0f64, 0.0f64)));

    let on_down = {
        let start_pos = start_pos.clone();
        move |e: leptos::ev::MouseEvent| {
            start_pos.replace((e.client_x() as f64, e.client_y() as f64));
            set_delta.set((0.0, 0.0));
            set_dragging.set(true);
        }
    };

    let on_move = {
        let start_pos = start_pos.clone();
        move |e: leptos::ev::MouseEvent| {
            if dragging.get() {
                let start = *start_pos.borrow();
                let dx = e.client_x() as f64 - start.0;
                let dy = e.client_y() as f64 - start.1;
                set_delta.set((dx, dy));
            }
        }
    };

    let on_up = move |_: leptos::ev::MouseEvent| {
        set_dragging.set(false);
    };

    (on_down, on_move, on_up, delta, dragging)
}
