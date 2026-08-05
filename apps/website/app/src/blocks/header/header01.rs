use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Header01() -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let menu_open = RwSignal::new(false);

    let _ = window_event_listener(leptos::ev::scroll, move |_| {
        let y = web_sys::window().unwrap().scroll_y().unwrap_or(0.0);
        scrolled.set(y > 10.0);
    });

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <header class=move || {
                let base = "sticky top-0 flex items-center justify-between px-6 py-4 border-b border-border transition-all duration-300";
                if scrolled.get() {
                    format!("{} bg-background/95 backdrop-blur-md shadow-sm", base)
                } else {
                    format!("{} bg-background/80 backdrop-blur-sm", base)
                }
            }>
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                    <span class="font-bold">"MontRS"</span>
                </div>
                <nav class="hidden md:flex items-center gap-6 text-sm">
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Features"</a>
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Pricing"</a>
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Docs"</a>
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"About"</a>
                </nav>
                <div class="flex items-center gap-3">
                    <button class="hidden md:inline-block text-sm text-muted-foreground hover:text-foreground">"Sign In"</button>
                    <button class="hidden md:inline-block rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                        "Get Started"
                    </button>
                    <button
                        on:click=move |_| menu_open.update(|v| *v = !*v)
                        class="md:hidden rounded-md p-2 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
                    >
                        <Icon glyph=if menu_open.get() { Glyph::X } else { Glyph::Menu } class="w-5 h-5" />
                    </button>
                </div>
            </header>
            <Show when=move || menu_open.get()>
                <div class="md:hidden border-b border-border bg-muted/50 p-4 space-y-2">
                    <a href="#" class="block rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"Features"</a>
                    <a href="#" class="block rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"Pricing"</a>
                    <a href="#" class="block rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"Docs"</a>
                    <a href="#" class="block rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"About"</a>
                    <div class="pt-2 space-y-2">
                        <button class="w-full text-sm text-muted-foreground hover:text-foreground">"Sign In"</button>
                        <button class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                            "Get Started"
                        </button>
                    </div>
                </div>
            </Show>
            <div class="p-12 text-center">
                <h2 class="text-3xl font-bold">"Sticky header with backdrop blur"</h2>
                <p class="mt-4 text-muted-foreground max-w-md mx-auto">"Scroll down to see the header opacity change. Tap the hamburger for mobile menu."</p>
            </div>
        </div>
    }
}
