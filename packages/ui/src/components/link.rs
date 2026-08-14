use leptos::prelude::*;

crate::variants! {
    Link {
        base: "inline-flex items-center underline-offset-4 hover:underline",
        variants: {
            variant: {
                Default: "text-primary",
                Muted: "text-muted-foreground",
                Destructive: "text-destructive",
            }
        }
    }
}

/// Link component with variant support.
///
/// Renders an anchor element with theme-aware styling.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Link href="/docs" variant=LinkVariant::Muted>"Documentation"</Link>
/// }
/// ```
#[component]
pub fn Link(
    href: String,
    #[prop(into, optional)] variant: Signal<LinkVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = LinkClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };

    view! {
        <a href=href class=merged data-name="Link">
            {children()}
        </a>
    }
}
