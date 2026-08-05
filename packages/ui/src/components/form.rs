use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Form(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_submit: Option<
        Callback<leptos::ev::SubmitEvent>,
    >,
    #[prop(optional)] novalidate: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("space-y-6", class.get());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        if novalidate {
            ev.prevent_default();
        }
        if let Some(cb) = on_submit {
            cb.run(ev);
        }
    };

    view! {
        <form
            class=merged
            data-name="Form"
            role="form"
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            novalidate=novalidate.then_some("")
            on:submit=handle_submit
        >
            {children()}
        </form>
    }
}

#[component]
pub fn FormField(
    name: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("space-y-2", class.get());

    view! {
        <div class=merged data-name="FormField" data-field-name=name>
            {children()}
        </div>
    }
}

#[component]
pub fn FormLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "text-sm font-medium leading-none \
             peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
            class.get()
        )
    };

    view! {
        <label class=merged data-name="FormLabel">
            {children()}
        </label>
    }
}

#[component]
pub fn FormControl(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <div class=merged data-name="FormControl">
            {children()}
        </div>
    }
}

#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <p class=merged data-name="FormDescription" id=crate::utils::Utils::use_random_id()>
            {children()}
        </p>
    }
}

#[component]
pub fn FormMessage(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged =
        move || cn!("text-sm font-medium text-destructive", class.get());

    let id = crate::utils::Utils::use_random_id();

    view! {
        <p
            class=merged
            data-name="FormMessage"
            id=id
            role="alert"
            aria-live="polite"
        >
            {children()}
        </p>
    }
}
