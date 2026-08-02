use leptos::prelude::*;
use montrs_ui::prelude::*;

#[component]
pub fn FaqSimple() -> impl IntoView {
    let faqs = vec![
        ("What is MontRS?", "MontRS is a full-stack Rust web framework built for compile-time correctness, explicit boundaries, and agent-first development."),
        ("How do I get started?", "Run `montrs new my-app` and follow the golden path. Check the docs at docs.montrs.com for tutorials."),
        ("Is it production ready?", "MontRS is actively developed and used in production by early adopters. The API is stabilizing."),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm divide-y divide-border">
            {faqs.into_iter().map(|(question, answer)| {
                view! {
                    <div class="p-6">
                        <h3 class="text-base font-semibold">{question}</h3>
                        <p class="mt-2 text-sm text-muted-foreground">{answer}</p>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}