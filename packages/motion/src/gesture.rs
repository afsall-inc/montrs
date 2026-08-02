use leptos::prelude::*;

/// Reactive hover state for an element.
/// Returns `(on_mouse_enter, on_mouse_leave, is_hovered)`.
pub fn use_hover() -> (impl Fn(leptos::ev::MouseEvent) + Clone, impl Fn(leptos::ev::MouseEvent) + Clone, ReadSignal<bool>) {
    let (hovered, set_hovered) = signal(false);
    let on_enter = move |_: leptos::ev::MouseEvent| set_hovered.set(true);
    let on_leave = move |_: leptos::ev::MouseEvent| set_hovered.set(false);
    (on_enter, on_leave, hovered)
}

/// Reactive press/tap state.
pub fn use_press() -> (impl Fn(leptos::ev::MouseEvent) + Clone, impl Fn(leptos::ev::MouseEvent) + Clone, ReadSignal<bool>) {
    let (pressed, set_pressed) = signal(false);
    let on_down = move |_: leptos::ev::MouseEvent| set_pressed.set(true);
    let on_up = move |_: leptos::ev::MouseEvent| set_pressed.set(false);
    (on_down, on_up, pressed)
}

/// Pan/drag tracking.
pub fn use_pan() -> (ReadSignal<(f64, f64)>, ReadSignal<bool>) {
    let (delta, _) = signal((0.0f64, 0.0f64));
    let (dragging, _) = signal(false);
    (delta, dragging)
}