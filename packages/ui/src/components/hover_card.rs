use crate::cn::*;
use leptos::prelude::*;

/// Hover card / tooltip that appears on hover.
///
/// Shows additional content when hovering over a trigger element.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <HoverCard>
///         <HoverCardTrigger>"Hover me"</HoverCardTrigger>
///         <HoverCardContent>"Extra info"</HoverCardContent>
///     </HoverCard>
/// }
/// ```
#[component]
pub fn HoverCard(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="HoverCard">
            {children()}
        </div>
    }
}

/// Trigger element that shows the hover card.
#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("HoverCardTrigger must be inside HoverCard");

    let merged = move || cn!("", class.get());

    view! {
        <div
            class=merged
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
            data-name="HoverCardTrigger"
        >
            {children()}
        </div>
    }
}

/// Hover card content.
#[component]
pub fn HoverCardContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("HoverCardContent must be inside HoverCard");

    let merged = move || {
        cn!(
            "z-50 w-64 rounded-md border bg-popover p-4 \
             text-popover-foreground shadow-md outline-none \
             data-[state=open]:animate-in data-[state=closed]:animate-out \
             data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
             data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
            class.get()
        )
    };

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="HoverCardContent"
        >
            {children()}
        </div>
    }
}
