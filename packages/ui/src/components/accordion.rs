use leptos::prelude::*;
use crate::cn::*;

/// Accordion container that manages expanded state for multiple items.
///
/// Supports multiple or single item expansion via `multiple` prop.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Accordion>
///         <AccordionItem value="item-1">
///             <AccordionTrigger>"Section 1"</AccordionTrigger>
///             <AccordionContent>"Content 1"</AccordionContent>
///         </AccordionItem>
///     </Accordion>
/// }
/// ```
#[component]
pub fn Accordion(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] multiple: bool,
    children: Children,
) -> impl IntoView {
    let expanded = RwSignal::<Vec<String>>::new(Vec::new());
    provide_context(expanded);
    provide_context(multiple);

    let merged = move || cn!("divide-y divide-border", class.get());

    view! {
        <div class=merged data-name="Accordion">
            {children()}
        </div>
    }
}

/// An individual accordion item.
#[component]
pub fn AccordionItem(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("border-b border-border", class.get());

    let expanded = use_context::<RwSignal<Vec<String>>>()
        .expect("AccordionItem must be inside Accordion");
    let value_for_is_open = value.clone();
    let is_open = move || expanded.with(|v| v.contains(&value_for_is_open));

    let value_for_toggle = value.clone();
    let toggle = move |_: leptos::ev::MouseEvent| {
        let multiple = use_context::<bool>().unwrap_or(false);
        expanded.update(|v| {
            if multiple {
                if let Some(pos) = v.iter().position(|x| x == &value_for_toggle) {
                    v.remove(pos);
                } else {
                    v.push(value_for_toggle.clone());
                }
            } else {
                if v.contains(&value_for_toggle) {
                    v.clear();
                } else {
                    v.clear();
                    v.push(value_for_toggle.clone());
                }
            }
        });
    };

    view! {
        <div class=merged data-name="AccordionItem">
            {children()}
        </div>
    }
}

/// The clickable trigger that toggles accordion item open/closed.
#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180",
        class.get()
    );

    view! {
        <button type="button" class=merged data-name="AccordionTrigger">
            {children()}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-4 w-4 shrink-0 transition-transform duration-200"
            >
                <path d="m6 9 6 6 6-6" />
            </svg>
        </button>
    }
}

/// The expandable content area of an accordion item.
#[component]
pub fn AccordionContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down",
        class.get()
    );

    view! {
        <div class=merged data-name="AccordionContent">
            <div class="pb-4 pt-0">{children()}</div>
        </div>
    }
}