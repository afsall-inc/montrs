use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav04() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let items = vec!["Home", "Search", "Settings", "Messages", "Notifications", "Profile", "Billing"];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <div class="relative mb-4">
                    <Icon glyph=Glyph::Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
                    <input
                        type="text"
                        placeholder="Search..."
                        on:input=move |ev| query.set(event_target_value(&ev))
                        class="w-full rounded-md border border-input bg-background pl-9 pr-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                </div>
                {items.iter().filter_map(|label| {
                    let q = query.get().to_lowercase();
                    if !q.is_empty() && !label.to_lowercase().contains(&q) { return None; }
                    Some(view! {
                        <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                            <Icon glyph=Glyph::LayoutDashboard class="w-4 h-4" />
                            {*label}
                        </a>
                    })
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}