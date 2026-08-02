use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn ThemeToggle(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_toggle: Option<Callback<()>>,
) -> impl IntoView {
    let merged = move || cn!("inline-flex h-9 w-9 items-center justify-center rounded-md border border-input bg-background text-sm font-medium hover:bg-accent hover:text-accent-foreground", class.get());
    let handle = move |_| {
        if let Some(cb) = on_toggle {
            cb.run(());
        }
    };
    view! {
        <button type="button" class=merged on:click=handle data-name="ThemeToggle">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="4" />
                <path d="M12 2v2" /><path d="M12 20v2" /><path d="m4.93 4.93 1.41 1.41" />
                <path d="m17.66 17.66 1.41 1.41" /><path d="M2 12h2" /><path d="M20 12h2" />
                <path d="m6.34 17.66-1.41 1.41" /><path d="m19.07 4.93-1.41 1.41" />
            </svg>
            <span class="sr-only">"Toggle theme"</span>
        </button>
    }
}