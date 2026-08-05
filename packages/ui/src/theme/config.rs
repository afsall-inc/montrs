use serde::{Deserialize, Serialize};

/// Configuration for the MontRS UI theming system.
/// Mirrors shadcn's components.json structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub style: String,
    pub tailwind: TailwindConfig,
    pub aliases: AliasesConfig,
    #[serde(default = "default_icon_library")]
    pub icon_library: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailwindConfig {
    pub css: String,
    pub toml: Option<String>,
    pub base_color: String,
    #[serde(default = "default_css_variables")]
    pub css_variables: bool,
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasesConfig {
    pub components: String,
    pub utils: String,
    pub ui: Option<String>,
    pub hooks: Option<String>,
}

fn default_icon_library() -> String {
    "montrs".to_string()
}

fn default_css_variables() -> bool {
    true
}

impl ComponentsConfig {
    pub fn default_neutral() -> Self {
        Self {
            schema: Some("https://ui.montrs.com/schema.json".into()),
            style: "default".into(),
            tailwind: TailwindConfig {
                css: "style/main.css".into(),
                toml: Some("tailwind.toml".into()),
                base_color: "neutral".into(),
                css_variables: true,
                prefix: String::new(),
            },
            aliases: AliasesConfig {
                components: "@/components".into(),
                utils: "@/lib/utils".into(),
                ui: Some("@/components/ui".into()),
                hooks: Some("@/hooks".into()),
            },
            icon_library: "montrs".into(),
        }
    }

    pub fn load() -> Option<Self> {
        let path = std::path::Path::new("components.json");
        if !path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("components.json", json)?;
        Ok(())
    }
}
