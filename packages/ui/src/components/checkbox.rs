use leptos::prelude::*;
use crate::cn::*;

/// Checkbox component with label.
///
/// Renders a styled checkbox input with an optional label.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Checkbox id="terms" label="Accept terms" />
/// }
/// ```
#[component]
pub fn Checkbox(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let merged = move || cn!(
        "peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
        class.get()
    );

    let input_id = id.unwrap_or_else(crate::utils::Utils::use_random_id);
    let state = move || if checked.get() { "checked" } else { "unchecked" };

    let toggle = move |_| checked.update(|v| *v = !*v);

    view! {
        <div class="flex items-center space-x-2">
            <button
                type="button"
                role="checkbox"
                id=input_id.clone()
                class=merged
                data-state=state
                aria-checked=checked
                disabled=disabled
                on:click=toggle
                data-name="Checkbox"
            >
                {move || if checked.get() {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24" height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="h-4 w-4"
                        >
                            <path d="M20 6 9 17l-5-5" />
                        </svg>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </button>
            {label.map(move |l| {
                let label_id = input_id.clone();
                view! {
                    <label
                        for=label_id
                        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                        {l}
                    </label>
                }
            })}
        </div>
    }
}