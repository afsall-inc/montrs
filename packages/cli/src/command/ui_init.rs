// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::{Context, Result};
use console::style;
use std::{fs, path::Path};

pub async fn run(
    base_color: Option<String>,
    accent_color: Option<String>,
    radius: Option<String>,
) -> Result<()> {
    println!("{} Initializing MontRS UI theme...", style("🎨").bold());

    let base_color = base_color.unwrap_or_else(|| "neutral".to_string());
    let accent_color = accent_color.unwrap_or_else(|| base_color.clone());
    let radius = radius.unwrap_or_else(|| "0.5rem".to_string());

    let css_path = Path::new("style/main.css");
    let toml_path = Path::new("tailwind.toml");
    let config_path = Path::new("components.json");

    // Generate CSS variables
    let css = generate_theme_css(&base_color, &accent_color, &radius);
    ensure_parent_dir(css_path)?;
    fs::write(css_path, &css).context("Failed to write style/main.css")?;
    println!(
        "  {} Wrote theme CSS to {}",
        style("✓").green().bold(),
        css_path.display()
    );

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

fn color_hue(name: &str) -> f64 {
    match name {
        "red" => 0.0,
        "orange" => 24.0,
        "amber" => 38.0,
        "yellow" => 50.0,
        "lime" => 90.0,
        "green" => 142.0,
        "emerald" => 160.0,
        "teal" => 180.0,
        "cyan" => 200.0,
        "sky" => 210.0,
        "blue" => 222.0,
        "indigo" => 240.0,
        "violet" => 270.0,
        "purple" => 288.0,
        "fuchsia" => 300.0,
        "pink" => 330.0,
        "rose" => 350.0,
        "stone" => 30.0,
        "slate" => 215.0,
        "zinc" => 240.0,
        _ => 222.0, // neutral/blue default
    }
}

fn generate_theme_css(
    base_color: &str,
    accent_color: &str,
    radius: &str,
) -> String {
    let base_hue = color_hue(base_color);
    let accent_hue = color_hue(accent_color);

    let mut css = String::from(
        "@tailwind base;\n@tailwind components;\n@tailwind \
         utilities;\n\n@layer base {\n",
    );

    // :root (light mode)
    css.push_str("    :root {\n");
    css.push_str(&format!("        --radius: {};\n", radius));

    // Light theme with base color
    let light = light_vars(base_hue, accent_hue);
    for (key, value) in &light {
        css.push_str(&format!("        --{}: {};\n", key, value));
    }
    css.push_str("    }\n\n");

    // .dark (dark mode)
    css.push_str("    .dark {\n");
    let dark = dark_vars(base_hue, accent_hue);
    for (key, value) in &dark {
        css.push_str(&format!("        --{}: {};\n", key, value));
    }
    css.push_str("    }\n");

    css.push_str(
        "}\n\n@layer base {\n    * { @apply border-border; }\n    body { \
         @apply bg-background text-foreground; font-feature-settings: \
         \"rlig\" 1, \"calt\" 1; }\n}\n\nhtml { scroll-behavior: smooth; \
         }\n\n@layer base {\n    *:focus-visible { @apply outline-none ring-2 \
         ring-ring ring-offset-2 ring-offset-background; }\n}\n\n::selection \
         { @apply bg-primary text-primary-foreground; }\n",
    );

    css
}

fn light_vars(base_hue: f64, accent_hue: f64) -> Vec<(&'static str, String)> {
    vec![
        ("background", format!("{} 0% 100%", base_hue)),
        ("foreground", format!("{} 84% 4.9%", base_hue)),
        ("card", format!("{} 0% 100%", base_hue)),
        ("card-foreground", format!("{} 84% 4.9%", base_hue)),
        ("popover", format!("{} 0% 100%", base_hue)),
        ("popover-foreground", format!("{} 84% 4.9%", base_hue)),
        ("primary", format!("{} 47.4% 11.2%", base_hue)),
        ("primary-foreground", "210 40% 98%".to_string()),
        ("secondary", format!("{} 40% 96.1%", base_hue)),
        ("secondary-foreground", format!("{} 47.4% 11.2%", base_hue)),
        ("muted", format!("{} 40% 96.1%", base_hue)),
        ("muted-foreground", format!("{} 16.3% 46.9%", base_hue)),
        ("accent", format!("{} 40% 96.1%", accent_hue)),
        ("accent-foreground", format!("{} 47.4% 11.2%", accent_hue)),
        ("destructive", "0 84.2% 60.2%".to_string()),
        ("destructive-foreground", "210 40% 98%".to_string()),
        ("border", format!("{} 31.8% 91.4%", base_hue)),
        ("input", format!("{} 31.8% 91.4%", base_hue)),
        ("ring", format!("{} 84% 4.9%", base_hue)),
    ]
}

fn dark_vars(base_hue: f64, accent_hue: f64) -> Vec<(&'static str, String)> {
    vec![
        ("background", format!("{} 84% 4.9%", base_hue)),
        ("foreground", "210 40% 98%".to_string()),
        ("card", format!("{} 84% 4.9%", base_hue)),
        ("card-foreground", "210 40% 98%".to_string()),
        ("popover", format!("{} 84% 4.9%", base_hue)),
        ("popover-foreground", "210 40% 98%".to_string()),
        ("primary", "210 40% 98%".to_string()),
        ("primary-foreground", format!("{} 47.4% 11.2%", base_hue)),
        ("secondary", format!("{} 32.6% 17.5%", base_hue)),
        ("secondary-foreground", "210 40% 98%".to_string()),
        ("muted", format!("{} 32.6% 17.5%", base_hue)),
        ("muted-foreground", "215 20.2% 65.1%".to_string()),
        ("accent", format!("{} 32.6% 17.5%", accent_hue)),
        ("accent-foreground", "210 40% 98%".to_string()),
        ("destructive", "0 62.8% 30.6%".to_string()),
        ("destructive-foreground", "210 40% 98%".to_string()),
        ("border", format!("{} 32.6% 17.5%", base_hue)),
        ("input", format!("{} 32.6% 17.5%", base_hue)),
        ("ring", "212.7 26.8% 83.9%".to_string()),
    ]
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
"#
    .to_string()
}
