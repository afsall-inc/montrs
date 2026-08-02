use leptos::prelude::*;
use crate::cn::*;

/// Form component with label, description, and message.
///
/// Provides a structured form field with validation display.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Form>
///         <FormField name="email">
///             <FormLabel>"Email"</FormLabel>
///             <FormControl>
///                 <input type="email" />
///             </FormControl>
///             <FormDescription>"Enter your email."</FormDescription>
///             <FormMessage>"Invalid email."</FormMessage>
///         </FormField>
///     </Form>
/// }
/// ```
#[component]
pub fn Form(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("space-y-6", class.get());

    view! {
        <form class=merged data-name="Form">
            {children()}
        </form>
    }
}

/// Form field wrapper.
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

/// Form label.
#[component]
pub fn FormLabel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
        class.get()
    );

    view! {
        <label class=merged data-name="FormLabel">
            {children()}
        </label>
    }
}

/// Form control wrapper.
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

/// Form description text.
#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm text-muted-foreground", class.get());

    view! {
        <p class=merged data-name="FormDescription">
            {children()}
        </p>
    }
}

/// Form validation message.
#[component]
pub fn FormMessage(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("text-sm font-medium text-destructive", class.get());

    view! {
        <p class=merged data-name="FormMessage">
            {children()}
        </p>
    }
}