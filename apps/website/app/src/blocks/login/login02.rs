use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Login02() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="grid grid-cols-1 md:grid-cols-2">
                <div class="p-8">
                    <h3 class="text-xl font-semibold">"Create account"</h3>
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
                    <p class="mt-2 text-sm text-muted-foreground text-center">"No credit card required. Start building today."</p>
                </div>
            </div>
        </div>
    }
}