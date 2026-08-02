use leptos::prelude::*;
use crate::cn::*;

/// Card component with optional header, content, and footer.
///
/// A versatile container for content with consistent styling.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Card>
///         <CardHeader>
///             <CardTitle>"Title"</CardTitle>
///             <CardDescription>"Description"</CardDescription>
///         </CardHeader>
///         <CardContent>"Content"</CardContent>
///         <CardFooter>"Footer"</CardFooter>
///     </Card>
/// }
/// ```
#[component]
pub fn Card(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "rounded-lg border bg-card text-card-foreground shadow-sm",
        class.get()
    );

    view! {
        <div class=merged data-name="Card">
            {children()}
        </div>
    }
}

/// Card header section.
#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col space-y-1.5 p-6", class.get());

    view! {
        <div class=merged data-name="CardHeader">
            {children()}
        </div>
    }
}

/// Card title.
#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-2xl font-semibold leading-none tracking-tight", class.get());

    view! {
        <h3 class=merged data-name="CardTitle">
            {children()}
        </h3>
    }
}

/// Card description.
#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <p class=merged data-name="CardDescription">
            {children()}
        </p>
    }
}

/// Card content area.
#[component]
pub fn CardContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("p-6 pt-0", class.get());

    view! {
        <div class=merged data-name="CardContent">
            {children()}
        </div>
    }
}

/// Card footer section.
#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center p-6 pt-0", class.get());

    view! {
        <div class=merged data-name="CardFooter">
            {children()}
        </div>
    }
}