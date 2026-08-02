use leptos::prelude::*;
use crate::cn::*;

/// Modal dialog with overlay.
///
/// Renders a centered dialog box with a backdrop overlay.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Dialog>
///         <DialogTrigger>"Open"</DialogTrigger>
///         <DialogOverlay />
///         <DialogContent>
///             <DialogHeader>
///                 <DialogTitle>"Title"</DialogTitle>
///             </DialogHeader>
///             <DialogFooter>
///                 <button>"Close"</button>
///             </DialogFooter>
///         </DialogContent>
///     </Dialog>
/// }
/// ```
#[component]
pub fn Dialog(
    #[prop(optional)] default_open: bool,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    provide_context(open);

    view! {
        <div data-name="Dialog">
            {children()}
        </div>
    }
}

/// Trigger button that opens the dialog.
#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("DialogTrigger must be inside Dialog");
    let toggle = move |_| open.set(true);

    let merged = move || cn!("", class.get());

    view! {
        <button type="button" class=merged on:click=toggle data-name="DialogTrigger">
            {children()}
        </button>
    }
}

/// Dialog overlay / backdrop.
#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("DialogOverlay must be inside Dialog");

    let merged = move || cn!(
        "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
        class.get()
    );

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            on:click=close
            data-name="DialogOverlay"
        />
    }
}

/// Dialog content wrapper.
#[component]
pub fn DialogContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("DialogContent must be inside Dialog");

    let merged = move || cn!(
        "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg",
        class.get()
    );

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="DialogContent"
        >
            {children()}
            <button
                type="button"
                class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                on:click=close
                data-name="DialogClose"
            >
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
                    <path d="M18 6 6 18" />
                    <path d="m6 6 12 12" />
                </svg>
                <span class="sr-only">"Close"</span>
            </button>
        </div>
    }
}

/// Dialog header.
#[component]
pub fn DialogHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col space-y-1.5 text-center sm:text-left", class.get());

    view! {
        <div class=merged data-name="DialogHeader">
            {children()}
        </div>
    }
}

/// Dialog footer.
#[component]
pub fn DialogFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2", class.get());

    view! {
        <div class=merged data-name="DialogFooter">
            {children()}
        </div>
    }
}

/// Dialog title.
#[component]
pub fn DialogTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-lg font-semibold leading-none tracking-tight", class.get());

    view! {
        <h2 class=merged data-name="DialogTitle">
            {children()}
        </h2>
    }
}

/// Dialog description.
#[component]
pub fn DialogDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <div class=merged data-name="DialogDescription">
            {children()}
        </div>
    }
}