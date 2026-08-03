use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let theme = use_theme();

    view! {
        <header class="sticky top-0 z-50 w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="mx-auto max-w-6xl flex h-16 items-center justify-between px-6 lg:px-8">
                <div class="flex items-center gap-6">
                    <a href="/" class="flex items-center gap-2 font-bold text-lg">
                        <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                        "MontRS"
                    </a>
                    <nav class="hidden md:flex items-center gap-6 text-sm">
                        <a href="/" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Home"
                        </a>
                        <a href="/icons" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Icons"
                        </a>
                        <a href="/components" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Components"
                        </a>
                        <a href="/blocks" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Blocks"
                        </a>
                        <a href="/motion" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Motion"
                        </a>
                        <a href="/animated-icons" class="text-muted-foreground hover:text-foreground transition-colors">
                            "Animated Icons"
                        </a>
                    </nav>
                </div>
                <div class="flex items-center gap-4">
                    <a href="https://github.com/montrs/montrs" target="_blank"
                        class="text-muted-foreground hover:text-foreground transition-colors"
                    >
                        <Icon glyph=Glyph::Globe class="w-5 h-5" />
                    </a>
                    <button
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 w-9"
                        on:click=move |_| toggle_theme()
                        aria-label="Toggle theme"
                    >
                        {move || match theme.get() {
                            ThemeMode::Light => view! { <Icon glyph=Glyph::Sun class="w-4 h-4" /> }.into_any(),
                            ThemeMode::Dark => view! { <Icon glyph=Glyph::Moon class="w-4 h-4" /> }.into_any(),
                            ThemeMode::System => view! { <Icon glyph=Glyph::Monitor class="w-4 h-4" /> }.into_any(),
                        }}
                    </button>
                </div>
            </div>
        </header>
    }
}