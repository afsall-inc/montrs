use leptos::prelude::*;
use crate::cn::*;

/// Tab navigation with panels.
///
/// Renders a tabbed interface with content panels.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Tabs default_value="account">
///         <TabsList>
///             <TabsTrigger value="account">"Account"</TabsTrigger>
///             <TabsTrigger value="password">"Password"</TabsTrigger>
///         </TabsList>
///         <TabsContent value="account">"Account settings"</TabsContent>
///         <TabsContent value="password">"Password settings"</TabsContent>
///     </Tabs>
/// }
/// ```
#[component]
pub fn Tabs(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] default_value: Option<String>,
    children: Children,
) -> impl IntoView {
    let active = RwSignal::new(default_value.unwrap_or_default());
    provide_context(active);

    let merged = move || cn!("", class.get());

    view! {
        <div class=merged data-name="Tabs">
            {children()}
        </div>
    }
}

/// Tabs list / navigation bar.
#[component]
pub fn TabsList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground",
        class.get()
    );

    view! {
        <div class=merged role="tablist" data-name="TabsList">
            {children()}
        </div>
    }
}

/// Tabs trigger button.
#[component]
pub fn TabsTrigger(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let active = use_context::<RwSignal<String>>()
        .expect("TabsTrigger must be inside Tabs");
    let value_for_is_active = value.clone();
    let is_active = move || active.get() == value_for_is_active;
    let value_for_select = value.clone();
    let select = move |_| active.set(value_for_select.clone());

    let is_active_for_merged = is_active.clone();
    let merged = move || {
        let base = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
        let active_class = if is_active_for_merged() {
            "bg-background text-foreground shadow-sm"
        } else {
            ""
        };
        cn!(base, active_class, class.get())
    };

    view! {
        <button
            type="button"
            role="tab"
            class=merged
            aria-selected=is_active.clone()
            data-state={
                let is_active = is_active.clone();
                move || if is_active() { "active" } else { "inactive" }
            }
            on:click=select
            data-name="TabsTrigger"
        >
            {children()}
        </button>
    }
}

/// Tabs content panel.
#[component]
pub fn TabsContent(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let active = use_context::<RwSignal<String>>()
        .expect("TabsContent must be inside Tabs");
    let value_for_is_active = value.clone();
    let is_active = move || active.get() == value_for_is_active;

    let merged = move || cn!(
        "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        class.get()
    );

    view! {
        <div
            role="tabpanel"
            class=merged
            data-state={
                let is_active = is_active.clone();
                move || if is_active() { "active" } else { "inactive" }
            }
            hidden={
                let is_active = is_active.clone();
                move || !is_active()
            }
            data-name="TabsContent"
        >
            {children()}
        </div>
    }
}