use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Attachment(
    filename: String,
    #[prop(into, optional)] file_size: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("flex items-center gap-2 rounded-md border bg-muted/50 px-3 py-2 text-sm", class.get());
    view! {
        <div class=merged data-name="Attachment">
            <span class="truncate font-medium">{filename}</span>
            {file_size.map(|s| view! { <span class="text-muted-foreground">{s}</span> })}
        </div>
    }
}