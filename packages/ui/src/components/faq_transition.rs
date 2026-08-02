use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn FaqTransition(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("divide-y divide-border rounded-lg border", class.get());
    view! {
        <div class=merged data-name="FaqTransition">
            {children()}
        </div>
    }
}

#[component]
pub fn FaqItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] question: Option<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || cn!("", class.get());
    let toggle = move |_| open.update(|v| *v = !*v);
    let question_text = question.clone();
    view! {
        <div class=merged data-name="FaqItem">
            <button
                type="button"
                class="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium hover:bg-muted/50"
                on:click=toggle
            >
                {question_text.map(|q| view! { <span>{q}</span> })}
                <svg
                    xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                    viewBox="0 0 24 24" fill="none" stroke="currentColor"
                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                    class=move || if open.get() { "rotate-180 transition-transform" } else { "transition-transform" }
                >
                    <path d="m6 9 6 6 6-6" />
                </svg>
            </button>
            <div class=move || {
                let base = "overflow-hidden transition-all duration-200";
                let state = if open.get() { "max-h-96 px-4 pb-3" } else { "max-h-0" };
                cn!(base, state)
            }>
                {children()}
            </div>
        </div>
    }
}