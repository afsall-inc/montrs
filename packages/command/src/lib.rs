//! Typed command registry and deterministic prefix search.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub shortcut: Option<String>,
}

impl Command {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self { id: id.into(), name: name.into(), keywords: Vec::new(), shortcut: None }
    }

    fn matches(&self, query: &str) -> bool {
        let query = query.to_ascii_lowercase();
        self.name.to_ascii_lowercase().starts_with(&query)
            || self.keywords.iter().any(|keyword| keyword.to_ascii_lowercase().starts_with(&query))
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn register(&mut self, command: Command) {
        self.commands.retain(|existing| existing.id != command.id);
        self.commands.push(command);
        self.commands.sort_by(|left, right| left.id.cmp(&right.id));
    }

    pub fn search(&self, query: &str) -> Vec<&Command> {
        self.commands.iter().filter(|command| command.matches(query)).collect()
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_names_and_keywords_deterministically() {
        let mut registry = CommandRegistry::default();
        let mut command = Command::new("settings", "Settings");
        command.keywords.push("preferences".into());
        registry.register(command);
        assert_eq!(registry.search("pref")[0].id, "settings");
    }
}
