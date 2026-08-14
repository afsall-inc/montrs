use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration01() -> impl IntoView {
    let copied = RwSignal::new(Option::<String>::None);
    let icons = vec![
        Glyph::Search,
        Glyph::Settings,
        Glyph::User,
        Glyph::Bell,
        Glyph::LayoutDashboard,
        Glyph::Mail,
        Glyph::Calendar,
        Glyph::Clock,
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <h3 class="text-sm font-semibold mb-4">"Icon Library — Click to copy name"</h3>
            <div class="grid grid-cols-4 gap-4">
                {icons.into_iter().map(|g| {
                    let name = format!("{:?}", g);
                    let name_clone = name.clone();
                    let name_for_copied = name.clone();
                    let click = move |_| {
                        copied.set(Some(name_clone.clone()));
                    };
                    let is_copied = move || copied.get().as_deref() == Some(&name_for_copied);
                    view! {
                        <button on:click=click class="flex flex-col items-center gap-2 rounded-lg border border-border bg-muted/50 p-4 hover:bg-muted hover:border-primary/30 active:scale-95 transition-all">
                            <Icon glyph=g class="w-6 h-6 text-foreground" />
                            <span class="text-xs text-muted-foreground">{name}</span>
                            <Show when=is_copied>
                                <span class="text-[10px] text-green-500">"Copied!"</span>
                            </Show>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
