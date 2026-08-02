use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav03() -> impl IntoView {
    let open_sections = RwSignal::new(vec![true, false, false]);
    let sections: Vec<(&str, Vec<&str>)> = vec![
        ("Getting Started", vec!["Introduction", "Installation", "Quickstart"]),
        ("Guides", vec!["Routing", "Components", "State Management"]),
        ("Advanced", vec!["Plugins", "Deployment", "Performance"]),
    ];

    let sections_clone = sections.clone();

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                {sections_clone.into_iter().enumerate().map(|(i, (title, items))| {
                    let t = title;
                    let is_open = move || open_sections.with(|s| s[i]);
                    let toggle = move |_| open_sections.update(|s| s[i] = !s[i]);
                    view! {
                        <div>
                            <button on:click=toggle class="flex w-full items-center justify-between rounded-md px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                                {t}
                                <Icon glyph=Glyph::ChevronDown class=move || {
                                    if is_open() { "w-4 h-4 transition-transform rotate-180" }
                                    else { "w-4 h-4 transition-transform" }
                                } />
                            </button>
                            <Show when=is_open>
                                <div class="ml-2 mt-1 space-y-1">
                                    {items.iter().map(|item| {
                                        view! {
                                            <a href="#" class="block rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">{*item}</a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </Show>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}