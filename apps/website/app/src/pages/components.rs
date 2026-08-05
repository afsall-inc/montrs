use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

montrs_ui::variants! {
    Badge {
        base: "inline-flex items-center font-semibold rounded-md border transition-colors w-fit",
        variants: {
            variant: {
                Default: "border-transparent shadow bg-primary text-primary-foreground",
                Secondary: "border-transparent bg-secondary text-secondary-foreground",
                Outline: "text-foreground border-border",
                Destructive: "border-transparent bg-destructive text-destructive-foreground",
            },
            size: {
                Default: "px-2.5 py-0.5 text-xs",
                Sm: "px-1.5 py-0.5 text-[10px]",
                Lg: "px-3 py-1 text-sm",
            }
        },
        component: {
            element: span
        }
    }
}

montrs_ui::variants! {
    Button {
        base: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground shadow hover:bg-primary/90",
                Secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                Outline: "border border-border bg-background hover:bg-accent",
                Ghost: "hover:bg-accent hover:text-accent-foreground",
                Destructive: "bg-destructive text-destructive-foreground shadow hover:bg-destructive/90",
            },
            size: {
                Default: "h-10 px-4 py-2",
                Sm: "h-9 rounded-md px-3",
                Lg: "h-11 rounded-md px-8",
                Icon: "h-10 w-10",
            }
        },
        component: {
            element: button
        }
    }
}

montrs_ui::variants! {
    Card {
        base: "rounded-lg border border-border bg-card text-card-foreground shadow-sm",
        variants: {
            variant: {
                Default: "",
                Interactive: "hover:shadow-md transition-shadow cursor-pointer",
            },
            size: {
                Default: "",
            }
        },
        component: {
            element: div
        }
    }
}

#[component]
pub fn Components() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
            <div class="mb-12">
                <h1 class="text-3xl font-bold">"Components"</h1>
                <p class="mt-2 text-muted-foreground">
                    "Pre-built UI components using montrs-ui and Tailwind CSS."
                </p>
            </div>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Badge"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                </div>
                <div class="flex flex-wrap gap-4 items-center mt-4">
                    <Badge size=BadgeSize::Sm>"Small"</Badge>
                    <Badge>"Default"</Badge>
                    <Badge size=BadgeSize::Lg>"Large"</Badge>
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Button"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <Button>"Default"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Outline>"Outline"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                </div>
                <div class="flex flex-wrap gap-4 items-center mt-4">
                    <Button size=ButtonSize::Sm>"Small"</Button>
                    <Button>"Default"</Button>
                    <Button size=ButtonSize::Lg>"Large"</Button>
                    <Button size=ButtonSize::Icon>
                        <Icon glyph=Glyph::Search class="w-4 h-4" />
                    </Button>
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Card"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    <Card>
                        <div class="p-6">
                            <h3 class="text-lg font-semibold">"Card Title"</h3>
                            <p class="mt-2 text-sm text-muted-foreground">
                                "This is a default card with some content."
                            </p>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Interactive>
                        <div class="p-6">
                            <h3 class="text-lg font-semibold">"Interactive"</h3>
                            <p class="mt-2 text-sm text-muted-foreground">
                                "Hover over this card to see the shadow effect."
                            </p>
                        </div>
                    </Card>
                    <Card>
                        <div class="p-6">
                            <div class="flex items-center gap-2 mb-4">
                                <Icon glyph=Glyph::Bell class="w-5 h-5 text-primary" />
                                <h3 class="text-lg font-semibold">"With Icon"</h3>
                            </div>
                            <p class="text-sm text-muted-foreground">
                                "Cards can include icons and other elements."
                            </p>
                        </div>
                    </Card>
                </div>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mb-6">"Icons in Components"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <Button>
                        <Icon glyph=Glyph::Plus class="mr-2 w-4 h-4" />
                        "Add Item"
                    </Button>
                    <Button variant=ButtonVariant::Outline>
                        <Icon glyph=Glyph::Settings class="mr-2 w-4 h-4" />
                        "Settings"
                    </Button>
                    <Button variant=ButtonVariant::Ghost>
                        <Icon glyph=Glyph::Trash2 class="mr-2 w-4 h-4" />
                        "Delete"
                    </Button>
                    <Button variant=ButtonVariant::Destructive>
                        <Icon glyph=Glyph::TriangleAlert class="mr-2 w-4 h-4" />
                        "Danger"
                    </Button>
                </div>
            </section>
        </div>
    }
}
