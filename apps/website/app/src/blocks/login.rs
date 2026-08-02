use leptos::prelude::*;
use leptos::ev::submit;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn LoginFormCard() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm">
            <div class="mb-6 text-center">
                <Icon glyph=Glyph::Blocks class="mx-auto w-10 h-10 text-primary" />
                <h3 class="mt-4 text-xl font-semibold text-card-foreground">"Welcome back"</h3>
                <p class="mt-2 text-sm text-muted-foreground">"Sign in to your account"</p>
            </div>
            <form class="space-y-4">
                <div>
                    <label for="email" class="block text-sm font-medium mb-1">"Email"</label>
                    <input
                        id="email"
                        type="email"
                        placeholder="m@example.com"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                </div>
                <div>
                    <label for="password" class="block text-sm font-medium mb-1">"Password"</label>
                    <input
                        id="password"
                        type="password"
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                </div>
                <button
                    type="submit"
                    class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
                >
                    "Sign In"
                </button>
            </form>
            <p class="mt-4 text-center text-sm text-muted-foreground">
                <a href="#" class="text-primary hover:underline">"Forgot password?"</a>
            </p>
        </div>
    }
}

#[component]
pub fn LoginSplit() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="grid grid-cols-1 md:grid-cols-2">
                <div class="p-8">
                    <h3 class="text-xl font-semibold text-card-foreground">"Create account"</h3>
                    <p class="mt-2 text-sm text-muted-foreground">"Enter your details below"</p>
                    <form class="mt-6 space-y-4">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label for="first" class="block text-sm font-medium mb-1">"First"</label>
                                <input id="first" class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                            </div>
                            <div>
                                <label for="last" class="block text-sm font-medium mb-1">"Last"</label>
                                <input id="last" class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                            </div>
                        </div>
                        <div>
                            <label for="email2" class="block text-sm font-medium mb-1">"Email"</label>
                            <input id="email2" type="email" class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <button type="submit" class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                            "Sign Up"
                        </button>
                    </form>
                </div>
                <div class="hidden md:flex flex-col items-center justify-center bg-muted p-8">
                    <Icon glyph=Glyph::Rocket class="w-16 h-16 text-primary mb-4" />
                    <h3 class="text-lg font-semibold">"Get started free"</h3>
                    <p class="mt-2 text-sm text-muted-foreground text-center">
                        "No credit card required. Start building today."
                    </p>
                </div>
            </div>
        </div>
    }
}