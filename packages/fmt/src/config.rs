use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatterSettings {
    pub max_width: usize,
    pub tab_spaces: usize,
    pub indentation_style: IndentationStyle,
    pub newline_style: NewlineStyle,
    pub view: ViewSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndentationStyle {
    Tabs,
    Spaces,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NewlineStyle {
    Unix,
    Windows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewSettings {
    pub closing_tag_style: ClosingTagStyle,
    pub attr_value_brace_style: AttrValueBraceStyle,
    pub macro_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClosingTagStyle {
    Preserve,
    SelfClosing,
    NonSelfClosing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttrValueBraceStyle {
    Always,
    WhenRequired,
    Never,
}

impl Default for FormatterSettings {
    fn default() -> Self {
        Self {
            max_width: 100,
            tab_spaces: 4,
            indentation_style: IndentationStyle::Spaces,
            newline_style: NewlineStyle::Unix,
            view: ViewSettings::default(),
        }
    }
}

impl FormatterSettings {
    /// Load settings using the "Cascade of Truth":
    /// 1. montrs-fmt.toml (if it exists)
    /// 2. [fmt] section in montrs.toml (if it exists)
    /// 3. Default settings
    pub fn load() -> Self {
        // Try montrs-fmt.toml first
        if let Ok(content) = std::fs::read_to_string("montrs-fmt.toml") {
            if let Ok(settings) = toml::from_str(&content) {
                return settings;
            }
        }

        // Try [fmt] section in montrs.toml
        if let Ok(content) = std::fs::read_to_string("montrs.toml") {
            if let Ok(value) = toml::from_str::<toml::Value>(&content) {
                if let Some(fmt_value) = value.get("fmt") {
                    if let Ok(settings) = fmt_value.clone().try_into() {
                        return settings;
                    }
                }
            }
        }

        Self::default()
    }
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            closing_tag_style: ClosingTagStyle::SelfClosing,
            attr_value_brace_style: AttrValueBraceStyle::WhenRequired,
            macro_names: vec!["view".to_string()],
        }
    }
}
