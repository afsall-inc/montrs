use crate::cn::*;
use leptos::{prelude::*, wasm_bindgen::JsCast};

#[derive(Clone)]
struct TabsContext {
    active: RwSignal<String>,
    tab_ids: RwSignal<Vec<String>>,
    focused_index: RwSignal<Option<usize>>,
}

#[component]
pub fn Tabs(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] default_value: Option<String>,
    children: Children,
) -> impl IntoView {
    let active = RwSignal::new(default_value.unwrap_or_default());
    let tab_ids = RwSignal::<Vec<String>>::new(Vec::new());
    let focused_index = RwSignal::<Option<usize>>::new(None);

    provide_context(TabsContext {
        active,
        tab_ids,
        focused_index,
    });

    let merged = move || cn!("", class.get());

    view! {
        <div class=merged data-name="Tabs">
            {children()}
        </div>
    }
}

fn focus_tab_by_id(id: &str) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(el) = doc.get_element_by_id(id) {
            if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.focus();
            }
        }
    }
}

#[component]
pub fn TabsList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<TabsContext>().expect("TabsList must be inside Tabs");

    let merged = move || {
        cn!(
            "inline-flex h-10 items-center justify-center rounded-md bg-muted \
             p-1 text-muted-foreground",
            class.get()
        )
    };

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        let key = ev.key();
        let tab_ids = ctx.tab_ids.with(|ids| ids.clone());
        let tab_count = tab_ids.len();
        if tab_count == 0 {
            return;
        }

        let new_idx = match key.as_str() {
            "ArrowRight" => {
                ev.prevent_default();
                if let Some(focused) = ctx.focused_index.get() {
                    Some((focused + 1) % tab_count)
                } else {
                    Some(0)
                }
            }
            "ArrowLeft" => {
                ev.prevent_default();
                if let Some(focused) = ctx.focused_index.get() {
                    if focused == 0 {
                        Some(tab_count - 1)
                    } else {
                        Some(focused - 1)
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
                Some(tab_count - 1)
            }
            _ => None,
        };

        if let Some(idx) = new_idx {
            ctx.focused_index.set(Some(idx));
            if let Some(tab_id) = tab_ids.get(idx) {
                focus_tab_by_id(tab_id);
            }
        }
    };

    view! {
        <div
            class=merged
            role="tablist"
            data-name="TabsList"
            on:keydown=on_key_down
        >
            {children()}
        </div>
    }
}

fn tab_id_for_value(value: &str) -> String {
    format!("tab-{}", value)
}

fn panel_id_for_value(value: &str) -> String {
    format!("tabpanel-{}", value)
}

#[component]
pub fn TabsTrigger(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<TabsContext>().expect("TabsTrigger must be inside Tabs");

    let tab_id = tab_id_for_value(&value);
    let panel_id = panel_id_for_value(&value);

    let tab_id_clone = tab_id.clone();
    {
        let ids = ctx.tab_ids.with(|ids| ids.clone());
        if !ids.contains(&tab_id_clone) {
            ctx.tab_ids.update(|ids| ids.push(tab_id_clone));
        }
    }

    let value_for_is_active = value.clone();
    let is_active = move || ctx.active.get() == value_for_is_active;
    let value_for_select = value.clone();
    let select = move |_| ctx.active.set(value_for_select.clone());

    let is_active_for_merged = is_active.clone();
    let is_active_for_tabindex = is_active.clone();
    let is_active_for_aria = is_active.clone();
    let is_active_for_state = is_active.clone();
    let merged = move || {
        let base = "inline-flex items-center justify-center whitespace-nowrap \
                    rounded-sm px-3 py-1.5 text-sm font-medium \
                    ring-offset-background transition-all \
                    focus-visible:outline-none focus-visible:ring-2 \
                    focus-visible:ring-ring focus-visible:ring-offset-2 \
                    disabled:pointer-events-none disabled:opacity-50";
        let active_class = if is_active_for_merged() {
            "bg-background text-foreground shadow-sm"
        } else {
            ""
        };
        cn!(base, active_class, class.get())
    };

    let tabindex = move || {
        if is_active_for_tabindex() { "0" } else { "-1" }
    };

    view! {
        <button
            type="button"
            role="tab"
            class=merged
            id=tab_id.clone()
            aria-selected=is_active_for_aria
            aria-controls=panel_id.clone()
            tabindex=tabindex
            data-state=move || if is_active_for_state() { "active" } else { "inactive" }
            on:click=select
            data-name="TabsTrigger"
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<TabsContext>().expect("TabsContent must be inside Tabs");

    let value_for_is_active = value.clone();
    let is_active = move || ctx.active.get() == value_for_is_active;

    let panel_id = panel_id_for_value(&value);
    let tab_id = tab_id_for_value(&value);

    let is_active_for_state = is_active.clone();
    let is_active_for_hidden = is_active.clone();

    let merged = move || {
        cn!(
            "mt-2 ring-offset-background focus-visible:outline-none \
             focus-visible:ring-2 focus-visible:ring-ring \
             focus-visible:ring-offset-2",
            class.get()
        )
    };

    view! {
        <div
            role="tabpanel"
            class=merged
            id=panel_id.clone()
            aria-labelledby=tab_id.clone()
            data-state=move || if is_active_for_state() { "active" } else { "inactive" }
            hidden=move || !is_active_for_hidden()
            data-name="TabsContent"
        >
            {children()}
        </div>
    }
}
