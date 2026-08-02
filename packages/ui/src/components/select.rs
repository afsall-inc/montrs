use leptos::prelude::*;
use crate::cn::*;

crate::variants! {
    Select {
        base: "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
        variants: {
            variant: {
                Default: "",
            }
        }
    }
}

/// Select dropdown component.
///
/// Renders a styled select trigger that opens a dropdown list of options.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Select value=selected_signal>
///         <SelectItem value="1">"Option 1"</SelectItem>
///         <SelectItem value="2">"Option 2"</SelectItem>
///     </Select>
/// }
/// ```
#[component]
pub fn Select(
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);
    provide_context(value);

    let merged = move || {
        let v = SelectVariant::Default;
        let c = SelectClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };

    let toggle = move |_| open.update(|v| *v = !*v);

    view! {
        <div class="relative" data-name="Select">
            <button
                type="button"
                role="combobox"
                class=merged
                aria-expanded=move || open.get()
                on:click=toggle
                data-name="SelectTrigger"
            >
                <span>{move || value.get().to_string()}</span>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24" height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-4 w-4 opacity-50"
                >
                    <path d="m6 9 6 6 6-6" />
                </svg>
            </button>
            <SelectContent>
                {children()}
            </SelectContent>
        </div>
    }
}

/// Select dropdown content.
#[component]
pub fn SelectContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("SelectContent must be inside Select");

    let merged = move || cn!(
        "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
        class.get()
    );

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="SelectContent"
        >
            <div class="max-h-96 overflow-y-auto">
                {children()}
            </div>
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="SelectBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

/// Select item option.
#[component]
pub fn SelectItem(
    value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let selected_value = use_context::<RwSignal<String>>()
        .expect("SelectItem must be inside Select");
    let open = use_context::<RwSignal<bool>>()
        .expect("SelectItem must be inside Select");

    let value_for_is_selected = value.clone();
    let is_selected = move || selected_value.get() == value_for_is_selected;
    let is_selected_for_aria = is_selected.clone();
    let is_selected_for_merged = is_selected.clone();
    let merged = move || {
        let base = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        let active = if is_selected_for_merged() { "bg-accent text-accent-foreground" } else { "" };
        cn!(base, active, class.get())
    };

    let select = move |_| {
        selected_value.set(value.clone());
        open.set(false);
    };

    view! {
        <div
            class=merged
            role="option"
            aria-selected=is_selected_for_aria
            on:click=select
            data-name="SelectItem"
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {move || if is_selected() {
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
            </span>
            {children()}
        </div>
    }
}

/// Select separator.
#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="SelectSeparator" />
    }
}

/// Select label.
#[component]
pub fn SelectLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("py-1.5 pl-8 pr-2 text-sm font-semibold", class.get());

    view! {
        <div class=merged data-name="SelectLabel">
            {children()}
        </div>
    }
}