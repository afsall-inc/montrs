//! Command palette (KBar-style) — inspired by leptos-kbar and cmdk.
//! Uses Trie-based prefix search, keyboard navigation, and hotkeys.

use crate::cn::*;
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// ============================================================================
// KBarAction
// ============================================================================

static NEXT_ACTION_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
pub struct KBarAction {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) shortcut: String,
    pub(crate) keywords: Vec<String>,
    pub(crate) perform: Callback<()>,
}

impl KBarAction {
    pub fn new(
        name: impl Into<String>,
        shortcut: impl Into<String>,
        keywords: Vec<String>,
        perform: Callback<()>,
    ) -> Self {
        Self {
            id: NEXT_ACTION_ID.fetch_add(1, Ordering::Relaxed),
            name: name.into(),
            shortcut: shortcut.into(),
            keywords,
            perform,
        }
    }
}

impl PartialEq for KBarAction {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

// ============================================================================
// KBarContext
// ============================================================================

#[derive(Clone, Copy)]
pub struct KBarContext {
    pub actions: RwSignal<Vec<Arc<KBarAction>>>,
    pub tree: RwSignal<Trie>,
}

pub fn use_kbar_context() -> KBarContext {
    expect_context::<KBarContext>()
}

// ============================================================================
// Trie (prefix tree)
// ============================================================================

#[derive(Debug, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self { Self { children: HashMap::new(), is_end_of_word: false } }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
    indexing: HashMap<String, Arc<KBarAction>>,
}

impl Default for Trie {
    fn default() -> Self { Self::new() }
}

impl Trie {
    pub fn new() -> Self { Self { root: TrieNode::new(), indexing: HashMap::new() } }

    pub fn batch_insert(actions: &[Arc<KBarAction>]) -> Self {
        let mut indexing = HashMap::new();
        for action in actions {
            indexing.insert(action.name.clone(), action.clone());
            for keyword in &action.keywords {
                indexing.insert(keyword.clone(), action.clone());
            }
        }
        let mut trie = Trie { root: TrieNode::new(), indexing };
        for action in actions {
            trie.insert(&action.name);
            for keyword in &action.keywords { trie.insert(keyword); }
        }
        trie
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() { current = current.children.entry(ch).or_insert_with(TrieNode::new); }
        current.is_end_of_word = true;
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<Arc<KBarAction>> {
        let mut current = &self.root;
        for ch in prefix.chars() {
            match current.children.get(&ch) { Some(node) => current = node, None => return Vec::new() }
        }
        let mut words = Vec::new();
        self.collect_words(current, prefix.to_string(), &mut words);
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
for word in &words {
            if let Some(action) = self.indexing.get(word)
                && seen.insert(action.id)
            {
                result.push(action.clone());
            }
        }
        result.sort_by_key(|a| a.id);
        result
    }

    fn collect_words(&self, node: &TrieNode, prefix: String, result: &mut Vec<String>) {
        if node.is_end_of_word { result.push(prefix.clone()); }
        for (ch, child) in &node.children {
            self.collect_words(child, format!("{prefix}{ch}"), result);
        }
    }
}

// ============================================================================
// CommandMenu component
// ============================================================================

#[component]
pub fn CommandMenu(
    #[prop(optional, default = "meta+k")] _hotkey: &'static str,
    #[prop(optional, default = "escape")] _escapekey: &'static str,
    actions: Vec<Arc<KBarAction>>,
    children: Children,
) -> impl IntoView {
    let show = RwSignal::new(false);
    let search = RwSignal::new(String::new());
    let selected_index = RwSignal::new(0);

    let tree = Trie::batch_insert(&actions);
    let actions_signal = RwSignal::new(actions);

    provide_context(KBarContext { actions: actions_signal, tree: RwSignal::new(tree) });

    let filtered = move || {
        let query = search.get().to_ascii_lowercase();
        let ctx = use_kbar_context();
        let tree = ctx.tree.get();
        tree.starts_with(&query)
    };

    let select_action = move |action: Arc<KBarAction>| {
        action.perform.run(());
        show.set(false);
        search.set(String::new());
    };

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => { ev.prevent_default(); selected_index.update(|i| *i = (*i + 1).min(filtered().len().saturating_sub(1))); }
            "ArrowUp" => { ev.prevent_default(); selected_index.update(|i| { *i = i.saturating_sub(1); }); }
            "Enter" => {
                ev.prevent_default();
                let items = filtered();
                let idx = selected_index.get();
                if idx < items.len() { select_action(items[idx].clone()); }
            }
            "Escape" => { show.set(false); }
            _ => {}
        }
    };

    view! {
        {children()}
        <Show when=move || show.get()>
            <div
                class="fixed inset-0 z-50 flex items-start justify-center pt-[15vh] bg-black/50"
                on:click=move |_| show.set(false)
            >
                <div
                    class="w-full max-w-lg rounded-lg border bg-popover shadow-2xl overflow-hidden"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <div class="flex items-center border-b px-3">
                        <svg class="mr-2 h-4 w-4 shrink-0 opacity-50" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
                        <input
                            class="flex h-11 w-full bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground"
                            placeholder="Type a command or search..."
                            prop:value=move || search.get()
                            on:input=move |ev| { search.set(event_target_value(&ev)); selected_index.set(0); }
                            on:keydown=on_keydown
                        />
                    </div>
                    <div class="max-h-[300px] overflow-y-auto p-1">
<For
                            each=move || filtered()
                            key=|action| action.id
                            children=move |action| {
                                let action_id = action.id;
                                let action_name = action.name.clone();
                                let action_shortcut = action.shortcut.clone();
                                let action_clone = action.clone();
                                let is_selected = move || selected_index.get() == filtered().iter().position(|a| a.id == action_id).unwrap_or(0);
                                view! {
                                    <div
                                        class=move || {
                                            let base = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none";
                                            let sel = if is_selected() { "bg-accent text-accent-foreground" } else { "text-foreground" };
                                            cn!(base, sel)
                                        }
                                        role="option"
                                        aria-selected=is_selected
                                        on:click=move |_| select_action(action_clone.clone())
                                        on:mouseenter=move |_| {
                                            let idx = filtered().iter().position(|a| a.id == action_id).unwrap_or(0);
                                            selected_index.set(idx);
                                        }
                                    >
<span class="flex-1">{action_name.to_string()}</span>
                                        {{
                                            let shortcut = action_shortcut.to_string();
                                            (!shortcut.is_empty()).then(move || {
                                                let parts: Vec<String> = shortcut.split('+').map(|p| p.to_string()).collect();
                                                view! {
                                                    <kbd class="ml-auto flex gap-1 text-xs text-muted-foreground">
                                                        {parts.into_iter().map(|p| view! { <span class="rounded border px-1 py-0.5">{p}</span> }).collect_view()}
                                                    </kbd>
                                                }
                                            })
                                        }}
                                    </div>
                                }
                            }
                        />
                        {move || {
                            let items = filtered();
                            if items.is_empty() {
                                Some(view! {
                                    <div class="py-6 text-center text-sm text-muted-foreground">
                                        "No results found."
                                    </div>
                                })
                            } else { None }
                        }}
                    </div>
                </div>
            </div>
        </Show>
    }
}
