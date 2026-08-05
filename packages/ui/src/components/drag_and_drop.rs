use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn DragAndDrop(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());
    view! {
        <div class=merged data-name="DragAndDrop">
            {children()}
        </div>
    }
}

#[component]
pub fn DraggableItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] drag_id: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("cursor-grab active:cursor-grabbing", class.get());
    view! {
        <div class=merged draggable="true" data-name="DraggableItem">
            {children()}
        </div>
    }
}
