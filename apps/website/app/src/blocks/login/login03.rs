use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Login03() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm max-w-sm mx-auto">
            <div class="mb-6 text-center">
                <Icon glyph=Glyph::Blocks class="mx-auto w-10 h-10 text-primary" />
                <h3 class="mt-4 text-xl font-semibold">"Sign in"</h3>
            </div>
            <div class="space-y-3">
                <button class="w-full flex items-center justify-center gap-2 rounded-md border border-border bg-background px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::GitBranch class="w-5 h-5" />
                    "Continue with GitHub"
                </button>
                <button class="w-full flex items-center justify-center gap-2 rounded-md border border-border bg-background px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::Globe class="w-5 h-5" />
                    "Continue with Google"
                </button>
                <button class="w-full flex items-center justify-center gap-2 rounded-md border border-border bg-background px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::MessageCircle class="w-5 h-5" />
                    "Continue with Discord"
                </button>
            </div>
            <div class="relative my-6">
                <div class="absolute inset-0 flex items-center"><span class="w-full border-t border-border" /></div>
                <div class="relative flex justify-center text-xs"><span class="bg-card px-2 text-muted-foreground">"Or continue with email"</span></div>
            </div>
            <form class="space-y-4">
                <input type="email" placeholder="m@example.com"
                    class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
                <button type="submit" class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                    "Continue"
                </button>
            </form>
        </div>
    }
}