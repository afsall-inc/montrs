use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Footer(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("border-t bg-background px-6 py-4", class.get());
    view! {
        <footer class=merged data-name="Footer">
            {children()}
        </footer>
    }
}
