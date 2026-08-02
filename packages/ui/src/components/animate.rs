use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Animate(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] animation: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let anim = animation.as_deref().unwrap_or("fade-in");
        cn!("animate-in", anim, class.get())
    };
    view! {
        <div class=merged data-name="Animate">
            {children()}
        </div>
    }
}