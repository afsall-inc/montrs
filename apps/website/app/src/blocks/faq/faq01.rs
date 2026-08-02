use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Faq01() -> impl IntoView {
    let open = RwSignal::new(Option::<usize>::None);
    let items = vec![
        ("What is MontRS?", "A full-stack Rust web framework for compile-time correctness and agent-first development."),
        ("How do I install it?", "Run `cargo add montrs` or use `montrs new my-app` to scaffold a new project."),
        ("Is it production ready?", "Yes — MontRS is used in production by early adopters. The API is stabilizing."),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm divide-y divide-border">
            {items.into_iter().enumerate().map(|(i, (q, a))| {
                let is_open = move || open.get() == Some(i);
                let toggle = move |_| open.set(if is_open() { None } else { Some(i) });
                view! {
                    <div class="p-4">
                        <button on:click=toggle class="flex w-full items-center justify-between text-left">
                            <span class="text-sm font-medium">{q}</span>
                            <Icon glyph=Glyph::ChevronDown class=move || {
                                if is_open() { "w-4 h-4 text-muted-foreground rotate-180 transition-transform" }
                                else { "w-4 h-4 text-muted-foreground transition-transform" }
                            } />
                        </button>
                        <Show when=is_open>
                            <p class="mt-3 text-sm text-muted-foreground">{a}</p>
                        </Show>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}