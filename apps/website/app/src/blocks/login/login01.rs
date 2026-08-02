use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Login01() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm max-w-sm mx-auto">
            <div class="mb-6 text-center">
                <Icon glyph=Glyph::Blocks class="mx-auto w-10 h-10 text-primary" />
                <h3 class="mt-4 text-xl font-semibold">"Welcome back"</h3>
                <p class="mt-2 text-sm text-muted-foreground">"Sign in to your account"</p>
            </div>
            <form class="space-y-4">
                <div>
                    <label for="email" class="block text-sm font-medium mb-1">"Email"</label>
                    <input id="email" type="email" placeholder="m@example.com"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
                </div>
                <div>
                    <label for="password" class="block text-sm font-medium mb-1">"Password"</label>
                    <input id="password" type="password"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" />
                </div>
                <button type="submit"
                    class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                    "Sign In"
                </button>
            </form>
            <p class="mt-4 text-center text-sm text-muted-foreground">
                <a href="#" class="text-primary hover:underline">"Forgot password?"</a>
            </p>
        </div>
    }
}