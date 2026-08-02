use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration03() -> impl IntoView {
    let row1 = vec!["Rust", "WASM", "Tailwind", "Docker", "Postgres", "Redis", "Nginx"];
    let row2 = vec!["GitHub", "Slack", "Discord", "Figma", "Notion", "Linear", "Vercel"];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm overflow-hidden">
            <h3 class="text-sm font-semibold mb-4">"Trusted by"</h3>
            <div class="space-y-4">
                <div class="flex gap-8 animate-marquee">
                    {row1.iter().chain(row1.iter()).map(|name| {
                        view! {
                            <div class="flex items-center gap-2 text-sm text-muted-foreground shrink-0">
                                <Icon glyph=Glyph::Blocks class="w-4 h-4" />
                                <span>{*name}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <div class="flex gap-8 animate-marquee-reverse">
                    {row2.iter().chain(row2.iter()).map(|name| {
                        view! {
                            <div class="flex items-center gap-2 text-sm text-muted-foreground shrink-0">
                                <Icon glyph=Glyph::Blocks class="w-4 h-4" />
                                <span>{*name}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}