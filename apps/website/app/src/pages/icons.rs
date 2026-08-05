use leptos::prelude::*;
use montrs_core::nav::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Icons() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();
    let search = RwSignal::new(query.get().get("search").unwrap_or_default());
    let size = RwSignal::new(query.get().get("size").unwrap_or_default());
    let color = RwSignal::new(query.get().get("color").unwrap_or_default());

    let icons = Memo::new(move |_| {
        let s = search.get();
        if s.is_empty() {
            Glyph::find("")
        } else {
            Glyph::find(&s)
        }
    });

    let icon_size = move || match size.get().as_str() {
        "sm" => "w-4 h-4",
        "lg" => "w-8 h-8",
        "xl" => "w-12 h-12",
        _ => "w-6 h-6",
    };

    let selected_icon = RwSignal::new(None::<Glyph>);

    view! {
        <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
            <div class="mb-8">
                <h1 class="text-3xl font-bold">"Icons"</h1>
                <p class="mt-2 text-muted-foreground">
                    {move || format!("{} icons available", Glyph::count())}
                </p>
            </div>

            <div class="mb-8 flex flex-wrap gap-4">
                <input
                    type="search"
                    placeholder="Search icons..."
                    class="flex-1 min-w-[200px] rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    prop:value=search
                    on:input={
                        let nav = navigate.clone();
                        move |e| {
                        let val = event_target_value(&e);
                        search.set(val.clone());
                        nav(&format!("/icons?search={}", val), Default::default());
                    }}
                />
                <select
                    class="rounded-md border border-input bg-background px-3 py-2 text-sm"
                    prop:value=size
                    on:change={
                        let nav = navigate.clone();
                        move |e| {
                        let val = event_target_value(&e);
                        size.set(val.clone());
                        nav(&format!("/icons?size={}", val), Default::default());
                    }}
                >
                    <option value="">"Default"</option>
                    <option value="sm">"Small"</option>
                    <option value="lg">"Large"</option>
                    <option value="xl">"XLarge"</option>
                </select>
                <input
                    type="color"
                    class="h-9 w-9 rounded-md border border-input cursor-pointer"
                    prop:value=color
                    on:input=move |e| {
                        let val = event_target_value(&e);
                        color.set(val.clone());
                    }
                />
            </div>

            <div class="grid grid-cols-6 sm:grid-cols-8 md:grid-cols-10 lg:grid-cols-12 gap-2"
                style:--icon-color=move || color.get()
            >
                <For
                    each=move || icons.get()
                    key=|g| *g
                    children=move |glyph| {
                        let name = glyph.name().to_string();
                        let kebab = glyph.kebab_name();
                        view! {
                            <button
                                class="flex flex-col items-center gap-1 rounded-lg border border-border p-3 hover:bg-accent transition-colors"
                                style="color: var(--icon-color)"
                                on:click=move |_| selected_icon.set(Some(glyph))
                                title=name.clone()
                            >
                                <Icon glyph=glyph class=icon_size() />
                                <span class="text-[10px] text-muted-foreground truncate w-full text-center">{kebab}</span>
                            </button>
                        }
                    }
                />
            </div>

            {move || selected_icon.get().map(|glyph| {
                let name = glyph.name().to_string();
                let svg = glyph.svg().to_string();
                view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm"
                        on:click=move |_| selected_icon.set(None)
                    >
                        <div class="rounded-lg border border-border bg-card p-8 shadow-lg max-w-md w-full mx-4"
                            on:click=|e| { e.stop_propagation(); }
                        >
                            <div class="flex justify-center mb-4">
                                <Icon glyph=glyph class="w-16 h-16" attr:style="color: var(--icon-color)" />
                            </div>
                            <h3 class="text-lg font-semibold text-center">{name.clone()}</h3>
                            <p class="text-sm text-muted-foreground text-center mt-1">{glyph.kebab_name()}</p>
                            <div class="mt-4 rounded-md bg-muted p-3">
                                <code class="text-xs break-all">{svg}</code>
                            </div>
                            <button
                                class="mt-4 w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
                                on:click=move |_| selected_icon.set(None)
                            >
                                "Close"
                            </button>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
