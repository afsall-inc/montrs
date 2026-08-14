use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration02() -> impl IntoView {
    let active = RwSignal::new("GitHub");
    let integrations = vec![
        ("GitHub", "Version control and CI/CD", Glyph::GitBranch),
        ("Slack", "Team communication", Glyph::MessageSquare),
        ("Discord", "Community chat", Glyph::MessageCircle),
        ("Docker", "Container deployment", Glyph::Container),
        ("Postgres", "Relational database", Glyph::Database),
        ("Redis", "In-memory cache", Glyph::Zap),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <h3 class="text-sm font-semibold mb-4">"Integrations — Click to select"</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {integrations.into_iter().map(|(name, desc, icon)| {
                    let l = name;
                    let is_active = move || active.get() == l;
                    let click = move |_| active.set(l);
                    view! {
                        <button on:click=click class=move || {
                            let base = "flex items-center gap-3 rounded-lg border p-4 text-left transition-all";
                            if is_active() {
                                format!("{} border-primary bg-primary/5", base)
                            } else {
                                format!("{} border-border hover:bg-muted hover:border-primary/30", base)
                            }
                        }>
                            <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-primary/10">
                                <Icon glyph=icon class="w-5 h-5 text-primary" />
                            </div>
                            <div>
                                <h4 class="text-sm font-medium">{name}</h4>
                                <p class="text-xs text-muted-foreground">{desc}</p>
                            </div>
                            <Show when=is_active>
                                <Icon glyph=Glyph::Check class="w-4 h-4 text-primary ml-auto shrink-0" />
                            </Show>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
