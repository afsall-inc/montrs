use leptos::prelude::*;
use crate::cn::*;

/// Collapsible section with a trigger that toggles content visibility.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Collapsible>
///         <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
///         <CollapsibleContent>"Hidden content"</CollapsibleContent>
///     </Collapsible>
/// }
/// ```
#[component]
pub fn Collapsible(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] default_open: bool,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    provide_context(open);

    let merged = move || cn!("", class.get());

    view! {
        <div class=merged data-name="Collapsible">
            {children()}
        </div>
    }
}

/// Trigger button that toggles collapsible content.
#[component]
pub fn CollapsibleTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("CollapsibleTrigger must be inside Collapsible");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || cn!(
        "flex w-full items-center justify-between py-2 text-sm font-medium [&[data-state=open]>svg]:rotate-180",
        class.get()
    );

    let state = move || if open.get() { "open" } else { "closed" };

    view! {
        <button type="button" class=merged data-state=state on:click=toggle data-name="CollapsibleTrigger">
            {children()}
        </button>
    }
}

/// Content area that expands/collapses.
#[component]
pub fn CollapsibleContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("CollapsibleContent must be inside Collapsible");

    let merged = move || cn!(
        "overflow-hidden data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down",
        class.get()
    );

    let state = move || if open.get() { "open" } else { "closed" };

    view! {
        <div
            class=merged
            data-state=state
            hidden=move || !open.get()
            data-name="CollapsibleContent"
        >
            {children()}
        </div>
    }
}