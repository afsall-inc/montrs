use leptos::prelude::*;
use montrs_ui::prelude::*;

#[component]
pub fn Faq02() -> impl IntoView {
    let open = RwSignal::new(Option::<usize>::None);
    let items = vec![
        (
            "What is MontRS?",
            "A full-stack Rust web framework for compile-time correctness.",
        ),
        (
            "How do I get started?",
            "Run `montrs new my-app` and follow the golden path.",
        ),
        (
            "Is it production ready?",
            "Yes — actively used by early adopters.",
        ),
        (
            "Does it support WASM?",
            "Yes, MontRS compiles to WASM for full-stack apps.",
        ),
        (
            "What about databases?",
            "MontRS ORM supports PostgreSQL, SQLite, and MySQL.",
        ),
        (
            "Is there a community?",
            "Join our Discord and GitHub discussions.",
        ),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm p-6">
            <h3 class="text-lg font-semibold mb-6">"Frequently Asked Questions"</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                {items.into_iter().enumerate().map(|(i, (q, a))| {
                    let is_open = move || open.get() == Some(i);
                    let toggle = move |_| open.set(if is_open() { None } else { Some(i) });
                    view! {
                        <div class="flex gap-3">
                            <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-bold text-primary">
                                {i + 1}
                            </span>
                            <div>
                                <button on:click=toggle class="text-left">
                                    <h4 class="text-sm font-medium hover:text-primary transition-colors">{q}</h4>
                                </button>
                                <Show when=is_open>
                                    <p class="mt-1 text-xs text-muted-foreground">{a}</p>
                                </Show>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
