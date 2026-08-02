use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Login04() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm max-w-md mx-auto">
            <div class="mb-6 text-center">
                <Icon glyph=Glyph::Blocks class="mx-auto w-10 h-10 text-primary" />
                <h3 class="mt-4 text-xl font-semibold">"Create your account"</h3>
                <p class="mt-2 text-sm text-muted-foreground">"Join thousands of developers building with MontRS."</p>
            </div>
            <form class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <label for="first" class="block text-sm font-medium mb-1">"First name"</label>
                        <input id="first" class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                    </div>
                    <div>
                        <label for="last" class="block text-sm font-medium mb-1">"Last name"</label>
                        <input id="last" class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                    </div>
                </div>
                <div>
                    <label for="email" class="block text-sm font-medium mb-1">"Email"</label>
                    <input id="email" type="email" placeholder="m@example.com"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground" />
                </div>
                <div>
                    <label for="password" class="block text-sm font-medium mb-1">"Password"</label>
                    <input id="password" type="password"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                </div>
                <div>
                    <label for="confirm" class="block text-sm font-medium mb-1">"Confirm password"</label>
                    <input id="confirm" type="password"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                </div>
                <button type="submit" class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                    "Create Account"
                </button>
            </form>
            <p class="mt-4 text-center text-sm text-muted-foreground">
                "Already have an account?" <a href="#" class="text-primary hover:underline">"Sign in"</a>
            </p>
        </div>
    }
}