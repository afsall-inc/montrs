use crate::cn::*;
use leptos::{prelude::*, wasm_bindgen::JsCast};

#[derive(Clone)]
struct AccordionItemData {
    value: String,
    trigger_id: String,
}

#[derive(Clone)]
struct AccordionContext {
    expanded: RwSignal<Vec<String>>,
    multiple: bool,
    items: RwSignal<Vec<AccordionItemData>>,
    focused_index: RwSignal<Option<usize>>,
}

#[component]
pub fn Accordion(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] multiple: bool,
    children: Children,
) -> impl IntoView {
    let expanded = RwSignal::<Vec<String>>::new(Vec::new());
    let items = RwSignal::<Vec<AccordionItemData>>::new(Vec::new());
    let focused_index = RwSignal::<Option<usize>>::new(None);

    provide_context(AccordionContext {
        expanded,
        multiple,
        items,
        focused_index,
    });

    let merged = move || cn!("divide-y divide-border", class.get());

    view! {
        <div class=merged data-name="Accordion">
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItem(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionContext>()
        .expect("AccordionItem must be inside Accordion");

    let trigger_id = crate::utils::Utils::use_random_id();
    let panel_id = crate::utils::Utils::use_random_id();

    {
        let entry = AccordionItemData {
            value: value.clone(),
            trigger_id: trigger_id.clone(),
        };
        ctx.items.update(|items| items.push(entry));
    }

    let item_ctx = AccordionItemContext {
        trigger_id,
        panel_id,
        value: value.clone(),
    };
    provide_context(item_ctx);

    let is_open = {
        let value = value.clone();
        move || ctx.expanded.with(|v| v.contains(&value))
    };

    let merged = move || cn!("border-b border-border", class.get());

    view! {
        <div class=merged data-name="AccordionItem" data-state=move || if is_open() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

#[derive(Clone)]
struct AccordionItemContext {
    trigger_id: String,
    panel_id: String,
    value: String,
}

fn focus_trigger_by_id(id: &str) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(el) = doc.get_element_by_id(id) {
            if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.focus();
            }
        }
    }
}

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionContext>()
        .expect("AccordionTrigger must be inside Accordion");
    let item_ctx = use_context::<AccordionItemContext>()
        .expect("AccordionTrigger must be inside AccordionItem");

    let value = item_ctx.value.clone();
    let trigger_id = item_ctx.trigger_id.clone();
    let panel_id = item_ctx.panel_id.clone();

    let value_for_is_open = value.clone();
    let is_open = move || ctx.expanded.with(|v| v.contains(&value_for_is_open));

    let value_for_toggle = value.clone();
    let toggle = move |_: leptos::ev::MouseEvent| {
        let multiple = ctx.multiple;
        ctx.expanded.update(|v| {
            if multiple {
                if let Some(pos) = v.iter().position(|x| x == &value_for_toggle)
                {
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

    let value_for_keydown = value.clone();
    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        let key = ev.key();
        let items = ctx.items.with(|items| items.clone());

        let current_idx = items
            .iter()
            .position(|entry| entry.value == value_for_keydown);

        let new_idx = match key.as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                if let Some(idx) = current_idx {
                    if idx + 1 < items.len() {
                        Some(idx + 1)
                    } else {
                        Some(0)
                    }
                } else {
                    Some(0)
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                if let Some(idx) = current_idx {
                    if idx > 0 {
                        Some(idx - 1)
                    } else {
                        Some(items.len() - 1)
                    }
                } else {
                    Some(0)
                }
            }
            "Home" => {
                ev.prevent_default();
                Some(0)
            }
            "End" => {
                ev.prevent_default();
                Some(items.len().saturating_sub(1))
            }
            _ => None,
        };

        if let Some(idx) = new_idx {
            ctx.focused_index.set(Some(idx));
            if let Some(entry) = items.get(idx) {
                focus_trigger_by_id(&entry.trigger_id);
            }
        }
    };

    let value_for_tabindex = value.clone();
    let tabindex = move || {
        let focused = ctx.focused_index.get();
        let items = ctx.items.with(|items| items.clone());
        let current_idx = items
            .iter()
            .position(|entry| entry.value == value_for_tabindex);
        if focused == current_idx || focused.is_none() && current_idx == Some(0)
        {
            "0"
        } else {
            "-1"
        }
    };

    let merged = move || {
        cn!(
            "flex flex-1 items-center justify-between py-4 text-sm \
             font-medium transition-all hover:underline \
             [&[data-state=open]>svg]:rotate-180",
            class.get()
        )
    };

    view! {
        <button
            type="button"
            class=merged
            data-name="AccordionTrigger"
            role="button"
            aria-expanded=is_open
            aria-controls=panel_id.clone()
            id=trigger_id.clone()
            tabindex=tabindex
            on:click=toggle
            on:keydown=on_key_down
        >
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

#[component]
pub fn AccordionContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionContext>()
        .expect("AccordionContent must be inside Accordion");
    let item_ctx = use_context::<AccordionItemContext>()
        .expect("AccordionContent must be inside AccordionItem");

    let value = item_ctx.value.clone();
    let trigger_id = item_ctx.trigger_id.clone();
    let panel_id = item_ctx.panel_id.clone();

    let is_open = move || ctx.expanded.with(|v| v.contains(&value));

    let merged = move || {
        cn!(
            "overflow-hidden text-sm data-[state=closed]:animate-accordion-up \
             data-[state=open]:animate-accordion-down",
            class.get()
        )
    };

    view! {
        <div
            class=merged
            data-name="AccordionContent"
            role="region"
            aria-labelledby=trigger_id.clone()
            id=panel_id.clone()
            hidden=move || !is_open()
        >
            <div class="pb-4 pt-0">{children()}</div>
        </div>
    }
}
