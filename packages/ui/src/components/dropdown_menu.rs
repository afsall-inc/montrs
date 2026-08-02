use leptos::prelude::*;
use crate::cn::*;

/// Dropdown menu with items.
///
/// Renders a trigger that opens a floating menu of items.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <DropdownMenu>
///         <DropdownMenuTrigger>"Menu"</DropdownMenuTrigger>
///         <DropdownMenuContent>
///             <DropdownMenuItem>"Profile"</DropdownMenuItem>
///             <DropdownMenuSeparator />
///             <DropdownMenuItem>"Logout"</DropdownMenuItem>
///         </DropdownMenuContent>
///     </DropdownMenu>
/// }
/// ```
#[component]
pub fn DropdownMenu(
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="DropdownMenu">
            {children()}
        </div>
    }
}

/// Trigger that opens the dropdown.
#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("DropdownMenuTrigger must be inside DropdownMenu");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || cn!("", class.get());

    view! {
        <button type="button" class=merged on:click=toggle data-name="DropdownMenuTrigger">
            {children()}
        </button>
    }
}

/// Dropdown menu content.
#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("DropdownMenuContent must be inside DropdownMenu");

    let merged = move || cn!(
        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
        class.get()
    );

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="DropdownMenuContent"
        >
            {children()}
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="DropdownMenuBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

/// Dropdown menu item.
#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        class.get()
    );

    let handle_click = move |_| {
        if let Some(cb) = on_select {
            cb.run(());
        }
    };

    view! {
        <div
            class=merged
            role="menuitem"
            on:click=handle_click
            data-name="DropdownMenuItem"
        >
            {children()}
        </div>
    }
}

/// Dropdown menu separator.
#[component]
pub fn DropdownMenuSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="DropdownMenuSeparator" />
    }
}

/// Dropdown menu label.
#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("px-2 py-1.5 text-sm font-semibold", class.get());

    view! {
        <div class=merged data-name="DropdownMenuLabel">
            {children()}
        </div>
    }
}

/// Dropdown menu radio group.
#[component]
pub fn DropdownMenuRadioGroup(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <div class=merged role="group" data-name="DropdownMenuRadioGroup">
            {children()}
        </div>
    }
}

/// Dropdown menu radio item.
#[component]
pub fn DropdownMenuRadioItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        class.get()
    );

    let toggle = move |_| checked.set(true);

    view! {
        <div
            class=merged
            role="menuitemradio"
            aria-checked=move || checked.get()
            on:click=toggle
            data-name="DropdownMenuRadioItem"
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
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
                            <circle cx="12" cy="12" r="2" />
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

/// Dropdown menu item with checkbox.
#[component]
pub fn DropdownMenuCheckboxItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        class.get()
    );

    let toggle = move |_| checked.update(|v| *v = !*v);

    view! {
        <div
            class=merged
            role="menuitemcheckbox"
            aria-checked=move || checked.get()
            on:click=toggle
            data-name="DropdownMenuCheckboxItem"
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
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
            </span>
            {children()}
        </div>
    }
}

/// Dropdown menu shortcut.
#[component]
pub fn DropdownMenuShortcut(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("ml-auto text-xs tracking-widest text-muted-foreground", class.get());

    view! {
        <span class=merged data-name="DropdownMenuShortcut">
            {children()}
        </span>
    }
}