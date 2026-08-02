use anyhow::{Result, Context};
use console::style;
use std::fs;
use std::path::Path;

pub async fn run(
    base_color: Option<String>,
    accent_color: Option<String>,
    radius: Option<String>,
) -> Result<()> {
    println!("{} Initializing MontRS UI theme...", style("🎨").bold());

    let base_color = base_color.unwrap_or_else(|| "neutral".to_string());
    let accent_color = accent_color.unwrap_or_default();
    let radius = radius.unwrap_or_else(|| "0.5rem".to_string());

    let css_path = Path::new("style/main.css");
    let toml_path = Path::new("tailwind.toml");
    let config_path = Path::new("components.json");

    // Generate CSS variables
    let css = generate_theme_css(&base_color, &accent_color, &radius);
    ensure_parent_dir(css_path)?;
    fs::write(css_path, &css).context("Failed to write style/main.css")?;
    println!("  {} Wrote theme CSS to {}", style("✓").green().bold(), css_path.display());

    // Generate tailwind.toml if it doesn't exist
    if !toml_path.exists() {
        let toml = generate_tailwind_toml();
        fs::write(toml_path, &toml).context("Failed to write tailwind.toml")?;
        println!("  {} Wrote tailwind.toml", style("✓").green().bold());
    }

    // Generate components.json
    let config = serde_json::json!({
        "$schema": "https://ui.montrs.com/schema.json",
        "style": "default",
        "tailwind": {
            "css": "style/main.css",
            "toml": "tailwind.toml",
            "base_color": base_color,
            "css_variables": true,
            "prefix": ""
        },
        "aliases": {
            "components": "@/components",
            "utils": "@/lib/utils",
            "ui": "@/components/ui",
            "hooks": "@/hooks"
        },
        "icon_library": "montrs"
    });
    fs::write(config_path, serde_json::to_string_pretty(&config)?)
        .context("Failed to write components.json")?;
    println!("  {} Wrote components.json", style("✓").green().bold());

    println!(
        "\n{} UI theme initialized! {}",
        style("✨").green().bold(),
        style("Use montrs serve to preview.").dim()
    );

    Ok(())
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn generate_theme_css(base_color: &str, accent_color: &str, radius: &str) -> String {
    let vars = theme_vars(base_color, accent_color);
    let mut css = String::from("@tailwind base;\n@tailwind components;\n@tailwind utilities;\n\n@layer base {\n");

    // :root (light mode)
    css.push_str("    :root {\n");
    css.push_str(&format!("        --radius: {};\n", radius));
    for (key, value) in &vars.light {
        css.push_str(&format!("        --{}: {};\n", key, value));
    }
    css.push_str("    }\n\n");

    // .dark (dark mode)
    css.push_str("    .dark {\n");
    for (key, value) in &vars.dark {
        css.push_str(&format!("        --{}: {};\n", key, value));
    }
    css.push_str("    }\n");

    css.push_str("}\n\n@layer base {\n    * { @apply border-border; }\n    body { @apply bg-background text-foreground; font-feature-settings: \"rlig\" 1, \"calt\" 1; }\n}\n\nhtml { scroll-behavior: smooth; }\n\n@layer base {\n    *:focus-visible { @apply outline-none ring-2 ring-ring ring-offset-2 ring-offset-background; }\n}\n\n::selection { @apply bg-primary text-primary-foreground; }\n");

    css
}

struct ThemeVars {
    light: Vec<(&'static str, &'static str)>,
    dark: Vec<(&'static str, &'static str)>,
}

fn theme_vars(base_color: &str, _accent_color: &str) -> ThemeVars {
    // Default neutral theme
    let light = vec![
        ("background", "0 0% 100%"),
        ("foreground", "222.2 84% 4.9%"),
        ("card", "0 0% 100%"),
        ("card-foreground", "222.2 84% 4.9%"),
        ("popover", "0 0% 100%"),
        ("popover-foreground", "222.2 84% 4.9%"),
        ("primary", "222.2 47.4% 11.2%"),
        ("primary-foreground", "210 40% 98%"),
        ("secondary", "210 40% 96.1%"),
        ("secondary-foreground", "222.2 47.4% 11.2%"),
        ("muted", "210 40% 96.1%"),
        ("muted-foreground", "215.4 16.3% 46.9%"),
        ("accent", "210 40% 96.1%"),
        ("accent-foreground", "222.2 47.4% 11.2%"),
        ("destructive", "0 84.2% 60.2%"),
        ("destructive-foreground", "210 40% 98%"),
        ("border", "214.3 31.8% 91.4%"),
        ("input", "214.3 31.8% 91.4%"),
        ("ring", "222.2 84% 4.9%"),
    ];

    let dark = vec![
        ("background", "222.2 84% 4.9%"),
        ("foreground", "210 40% 98%"),
        ("card", "222.2 84% 4.9%"),
        ("card-foreground", "210 40% 98%"),
        ("popover", "222.2 84% 4.9%"),
        ("popover-foreground", "210 40% 98%"),
        ("primary", "210 40% 98%"),
        ("primary-foreground", "222.2 47.4% 11.2%"),
        ("secondary", "217.2 32.6% 17.5%"),
        ("secondary-foreground", "210 40% 98%"),
        ("muted", "217.2 32.6% 17.5%"),
        ("muted-foreground", "215 20.2% 65.1%"),
        ("accent", "217.2 32.6% 17.5%"),
        ("accent-foreground", "210 40% 98%"),
        ("destructive", "0 62.8% 30.6%"),
        ("destructive-foreground", "210 40% 98%"),
        ("border", "217.2 32.6% 17.5%"),
        ("input", "217.2 32.6% 17.5%"),
        ("ring", "212.7 26.8% 83.9%"),
    ];

    // TODO: Support different base colors and accent colors from montrs-ui's theme::colors
    let _ = base_color;
    let _ = _accent_color;

    ThemeVars { light, dark }
}

fn generate_tailwind_toml() -> String {
    r#"# Tailwind CSS Configuration for MontRS
# This file is automatically converted to tailwind.config.js by montrs

content = [
    "src/**/*.rs",
    "src/**/*.html",
    "index.html"
]

[theme]
[theme.container]
center = true
padding = "2rem"

[theme.screens]
"2xl" = "1400px"

[theme.extend]
[theme.extend.colors]
border = "hsl(var(--border))"
input = "hsl(var(--input))"
ring = "hsl(var(--ring))"
background = "hsl(var(--background))"
foreground = "hsl(var(--foreground))"

[theme.extend.colors.primary]
DEFAULT = "hsl(var(--primary))"
foreground = "hsl(var(--primary-foreground))"

[theme.extend.colors.secondary]
DEFAULT = "hsl(var(--secondary))"
foreground = "hsl(var(--secondary-foreground))"

[theme.extend.colors.destructive]
DEFAULT = "hsl(var(--destructive))"
foreground = "hsl(var(--destructive-foreground))"

[theme.extend.colors.muted]
DEFAULT = "hsl(var(--muted))"
foreground = "hsl(var(--muted-foreground))"

[theme.extend.colors.accent]
DEFAULT = "hsl(var(--accent))"
foreground = "hsl(var(--accent-foreground))"

[theme.extend.colors.popover]
DEFAULT = "hsl(var(--popover))"
foreground = "hsl(var(--popover-foreground))"

[theme.extend.colors.card]
DEFAULT = "hsl(var(--card))"
foreground = "hsl(var(--card-foreground))"

[theme.extend.borderRadius]
lg = "var(--radius)"
md = "calc(var(--radius) - 2px)"
sm = "calc(var(--radius) - 4px)"
"#.to_string()
}