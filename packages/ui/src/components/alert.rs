use leptos::prelude::*;
use crate::cn::*;

crate::variants! {
    Alert {
        base: "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground",
        variants: {
            variant: {
                Default: "bg-background text-foreground",
                Destructive: "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive",
            }
        }
    }
}

/// Alert component for displaying messages.
///
/// Renders an alert with optional title, description, and icon.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Alert variant=AlertVariant::Destructive>
///         <AlertTitle>"Error"</AlertTitle>
///         <AlertDescription>"Something went wrong."</AlertDescription>
///     </Alert>
/// }
/// ```
#[component]
pub fn Alert(
    #[prop(into, optional)] variant: Signal<AlertVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = AlertClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };

    view! {
        <div role="alert" class=merged data-name="Alert">
            {children()}
        </div>
    }
}

/// Alert title.
#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("mb-1 font-medium leading-none tracking-tight", class.get());

    view! {
        <h5 class=merged data-name="AlertTitle">
            {children()}
        </h5>
    }
}

/// Alert description.
#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm [&_p]:leading-relaxed", class.get());

    view! {
        <div class=merged data-name="AlertDescription">
            {children()}
        </div>
    }
}