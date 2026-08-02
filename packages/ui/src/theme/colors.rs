use std::collections::HashMap;

/// A complete set of CSS variable values for a theme (light and dark).
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub light: HashMap<&'static str, &'static str>,
    pub dark: HashMap<&'static str, &'static str>,
}

/// Returns the 7 base color families.
pub fn base_colors() -> HashMap<&'static str, ThemeColors> {
    let mut colors = HashMap::new();
    colors.insert("neutral", neutral());
    colors.insert("stone", stone());
    colors.insert("zinc", zinc());
    colors.insert("mauve", mauve());
    colors.insert("olive", olive());
    colors.insert("mist", mist());
    colors.insert("taupe", taupe());
    colors
}

/// Returns the 14+ accent color themes.
pub fn accent_colors() -> HashMap<&'static str, ThemeColors> {
    let mut colors = HashMap::new();
    colors.insert("amber", accent_amber());
    colors.insert("blue", accent_blue());
    colors.insert("cyan", accent_cyan());
    colors.insert("emerald", accent_emerald());
    colors.insert("green", accent_green());
    colors.insert("indigo", accent_indigo());
    colors.insert("lime", accent_lime());
    colors.insert("orange", accent_orange());
    colors.insert("pink", accent_pink());
    colors.insert("purple", accent_purple());
    colors.insert("red", accent_red());
    colors.insert("rose", accent_rose());
    colors.insert("sky", accent_sky());
    colors.insert("teal", accent_teal());
    colors.insert("violet", accent_violet());
    colors.insert("yellow", accent_yellow());
    colors
}

/// Merge accent colors into a base color theme, producing overridden CSS vars.
pub fn merge_accent(base: &ThemeColors, accent: &ThemeColors) -> ThemeColors {
    let mut light = base.light.clone();
    let mut dark = base.dark.clone();

    for (key, value) in &accent.light {
        if key.starts_with("primary") || key.starts_with("chart") || key.starts_with("sidebar-primary") {
            light.insert(key, value);
        }
    }
    for (key, value) in &accent.dark {
        if key.starts_with("primary") || key.starts_with("chart") || key.starts_with("sidebar-primary") {
            dark.insert(key, value);
        }
    }

    ThemeColors { light, dark }
}

/// Generate CSS string from theme colors.
pub fn theme_to_css(theme: &ThemeColors, radius: Option<&str>) -> String {
    let radius = radius.unwrap_or("0.5rem");
    let mut css = String::new();

    css.push_str(":root {\n");
    css.push_str(&format!("    --radius: {};\n", radius));
    for (key, value) in &theme.light {
        css.push_str(&format!("    --{}: {};\n", key, value));
    }
    css.push_str("}\n\n");

    css.push_str(".dark {\n");
    for (key, value) in &theme.dark {
        css.push_str(&format!("    --{}: {};\n", key, value));
    }
    css.push_str("}\n");

    css
}

fn neutral() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
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
            ("chart-1", "12 76% 61%"),
            ("chart-2", "173 58% 39%"),
            ("chart-3", "197 37% 24%"),
            ("chart-4", "43 74% 66%"),
            ("chart-5", "27 87% 67%"),
        ]),
        dark: HashMap::from([
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
            ("chart-1", "220 70% 50%"),
            ("chart-2", "160 60% 45%"),
            ("chart-3", "30 80% 55%"),
            ("chart-4", "280 65% 60%"),
            ("chart-5", "340 75% 55%"),
        ]),
    }
}

fn stone() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("background", "0 0% 100%"),
            ("foreground", "20 14.3% 4.1%"),
            ("card", "0 0% 100%"),
            ("card-foreground", "20 14.3% 4.1%"),
            ("popover", "0 0% 100%"),
            ("popover-foreground", "20 14.3% 4.1%"),
            ("primary", "24 9.8% 10%"),
            ("primary-foreground", "60 9.1% 97.8%"),
            ("secondary", "60 4.8% 95.9%"),
            ("secondary-foreground", "24 9.8% 10%"),
            ("muted", "60 4.8% 95.9%"),
            ("muted-foreground", "25 5.3% 44.7%"),
            ("accent", "60 4.8% 95.9%"),
            ("accent-foreground", "24 9.8% 10%"),
            ("destructive", "0 84.2% 60.2%"),
            ("destructive-foreground", "60 9.1% 97.8%"),
            ("border", "20 5.9% 90%"),
            ("input", "20 5.9% 90%"),
            ("ring", "20 14.3% 4.1%"),
            ("chart-1", "12 76% 61%"),
            ("chart-2", "173 58% 39%"),
            ("chart-3", "197 37% 24%"),
            ("chart-4", "43 74% 66%"),
            ("chart-5", "27 87% 67%"),
        ]),
        dark: HashMap::from([
            ("background", "20 14.3% 4.1%"),
            ("foreground", "60 9.1% 97.8%"),
            ("card", "20 14.3% 4.1%"),
            ("card-foreground", "60 9.1% 97.8%"),
            ("popover", "20 14.3% 4.1%"),
            ("popover-foreground", "60 9.1% 97.8%"),
            ("primary", "60 9.1% 97.8%"),
            ("primary-foreground", "24 9.8% 10%"),
            ("secondary", "12 6.5% 15.1%"),
            ("secondary-foreground", "60 9.1% 97.8%"),
            ("muted", "12 6.5% 15.1%"),
            ("muted-foreground", "24 5.4% 63.9%"),
            ("accent", "12 6.5% 15.1%"),
            ("accent-foreground", "60 9.1% 97.8%"),
            ("destructive", "0 62.8% 30.6%"),
            ("destructive-foreground", "60 9.1% 97.8%"),
            ("border", "12 6.5% 15.1%"),
            ("input", "12 6.5% 15.1%"),
            ("ring", "24 5.7% 82.9%"),
            ("chart-1", "220 70% 50%"),
            ("chart-2", "160 60% 45%"),
            ("chart-3", "30 80% 55%"),
            ("chart-4", "280 65% 60%"),
            ("chart-5", "340 75% 55%"),
        ]),
    }
}

fn zinc() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("background", "0 0% 100%"),
            ("foreground", "240 10% 3.9%"),
            ("card", "0 0% 100%"),
            ("card-foreground", "240 10% 3.9%"),
            ("popover", "0 0% 100%"),
            ("popover-foreground", "240 10% 3.9%"),
            ("primary", "240 5.9% 10%"),
            ("primary-foreground", "0 0% 98%"),
            ("secondary", "240 4.8% 95.9%"),
            ("secondary-foreground", "240 5.9% 10%"),
            ("muted", "240 4.8% 95.9%"),
            ("muted-foreground", "240 3.8% 46.1%"),
            ("accent", "240 4.8% 95.9%"),
            ("accent-foreground", "240 5.9% 10%"),
            ("destructive", "0 84.2% 60.2%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "240 5.9% 90%"),
            ("input", "240 5.9% 90%"),
            ("ring", "240 5.9% 10%"),
            ("chart-1", "12 76% 61%"),
            ("chart-2", "173 58% 39%"),
            ("chart-3", "197 37% 24%"),
            ("chart-4", "43 74% 66%"),
            ("chart-5", "27 87% 67%"),
        ]),
        dark: HashMap::from([
            ("background", "240 10% 3.9%"),
            ("foreground", "0 0% 98%"),
            ("card", "240 10% 3.9%"),
            ("card-foreground", "0 0% 98%"),
            ("popover", "240 10% 3.9%"),
            ("popover-foreground", "0 0% 98%"),
            ("primary", "0 0% 98%"),
            ("primary-foreground", "240 5.9% 10%"),
            ("secondary", "240 3.7% 15.9%"),
            ("secondary-foreground", "0 0% 98%"),
            ("muted", "240 3.7% 15.9%"),
            ("muted-foreground", "240 5% 64.9%"),
            ("accent", "240 3.7% 15.9%"),
            ("accent-foreground", "0 0% 98%"),
            ("destructive", "0 62.8% 30.6%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "240 3.7% 15.9%"),
            ("input", "240 3.7% 15.9%"),
            ("ring", "240 4.9% 83.9%"),
            ("chart-1", "220 70% 50%"),
            ("chart-2", "160 60% 45%"),
            ("chart-3", "30 80% 55%"),
            ("chart-4", "280 65% 60%"),
            ("chart-5", "340 75% 55%"),
        ]),
    }
}

fn mauve() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("background", "0 0% 100%"),
            ("foreground", "300 10% 3.9%"),
            ("card", "0 0% 100%"),
            ("card-foreground", "300 10% 3.9%"),
            ("popover", "0 0% 100%"),
            ("popover-foreground", "300 10% 3.9%"),
            ("primary", "300 5.9% 10%"),
            ("primary-foreground", "0 0% 98%"),
            ("secondary", "300 4.8% 95.9%"),
            ("secondary-foreground", "300 5.9% 10%"),
            ("muted", "300 4.8% 95.9%"),
            ("muted-foreground", "300 3.8% 46.1%"),
            ("accent", "300 4.8% 95.9%"),
            ("accent-foreground", "300 5.9% 10%"),
            ("destructive", "0 84.2% 60.2%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "300 5.9% 90%"),
            ("input", "300 5.9% 90%"),
            ("ring", "300 5.9% 10%"),
        ]),
        dark: HashMap::from([
            ("background", "300 10% 3.9%"),
            ("foreground", "0 0% 98%"),
            ("card", "300 10% 3.9%"),
            ("card-foreground", "0 0% 98%"),
            ("popover", "300 10% 3.9%"),
            ("popover-foreground", "0 0% 98%"),
            ("primary", "0 0% 98%"),
            ("primary-foreground", "300 5.9% 10%"),
            ("secondary", "300 3.7% 15.9%"),
            ("secondary-foreground", "0 0% 98%"),
            ("muted", "300 3.7% 15.9%"),
            ("muted-foreground", "300 5% 64.9%"),
            ("accent", "300 3.7% 15.9%"),
            ("accent-foreground", "0 0% 98%"),
            ("destructive", "0 62.8% 30.6%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "300 3.7% 15.9%"),
            ("input", "300 3.7% 15.9%"),
            ("ring", "300 4.9% 83.9%"),
        ]),
    }
}

fn olive() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("background", "0 0% 100%"),
            ("foreground", "120 10% 3.9%"),
            ("card", "0 0% 100%"),
            ("card-foreground", "120 10% 3.9%"),
            ("popover", "0 0% 100%"),
            ("popover-foreground", "120 10% 3.9%"),
            ("primary", "120 5.9% 10%"),
            ("primary-foreground", "0 0% 98%"),
            ("secondary", "120 4.8% 95.9%"),
            ("secondary-foreground", "120 5.9% 10%"),
            ("muted", "120 4.8% 95.9%"),
            ("muted-foreground", "120 3.8% 46.1%"),
            ("accent", "120 4.8% 95.9%"),
            ("accent-foreground", "120 5.9% 10%"),
            ("destructive", "0 84.2% 60.2%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "120 5.9% 90%"),
            ("input", "120 5.9% 90%"),
            ("ring", "120 5.9% 10%"),
        ]),
        dark: HashMap::from([
            ("background", "120 10% 3.9%"),
            ("foreground", "0 0% 98%"),
            ("card", "120 10% 3.9%"),
            ("card-foreground", "0 0% 98%"),
            ("popover", "120 10% 3.9%"),
            ("popover-foreground", "0 0% 98%"),
            ("primary", "0 0% 98%"),
            ("primary-foreground", "120 5.9% 10%"),
            ("secondary", "120 3.7% 15.9%"),
            ("secondary-foreground", "0 0% 98%"),
            ("muted", "120 3.7% 15.9%"),
            ("muted-foreground", "120 5% 64.9%"),
            ("accent", "120 3.7% 15.9%"),
            ("accent-foreground", "0 0% 98%"),
            ("destructive", "0 62.8% 30.6%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "120 3.7% 15.9%"),
            ("input", "120 3.7% 15.9%"),
            ("ring", "120 4.9% 83.9%"),
        ]),
    }
}

fn mist() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("background", "0 0% 100%"),
            ("foreground", "210 10% 3.9%"),
            ("card", "0 0% 100%"),
            ("card-foreground", "210 10% 3.9%"),
            ("popover", "0 0% 100%"),
            ("popover-foreground", "210 10% 3.9%"),
            ("primary", "210 5.9% 10%"),
            ("primary-foreground", "0 0% 98%"),
            ("secondary", "210 4.8% 95.9%"),
            ("secondary-foreground", "210 5.9% 10%"),
            ("muted", "210 4.8% 95.9%"),
            ("muted-foreground", "210 3.8% 46.1%"),
            ("accent", "210 4.8% 95.9%"),
            ("accent-foreground", "210 5.9% 10%"),
            ("destructive", "0 84.2% 60.2%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "210 5.9% 90%"),
            ("input", "210 5.9% 90%"),
            ("ring", "210 5.9% 10%"),
        ]),
        dark: HashMap::from([
            ("background", "210 10% 3.9%"),
            ("foreground", "0 0% 98%"),
            ("card", "210 10% 3.9%"),
            ("card-foreground", "0 0% 98%"),
            ("popover", "210 10% 3.9%"),
            ("popover-foreground", "0 0% 98%"),
            ("primary", "0 0% 98%"),
            ("primary-foreground", "210 5.9% 10%"),
            ("secondary", "210 3.7% 15.9%"),
            ("secondary-foreground", "0 0% 98%"),
            ("muted", "210 3.7% 15.9%"),
            ("muted-foreground", "210 5% 64.9%"),
            ("accent", "210 3.7% 15.9%"),
            ("accent-foreground", "0 0% 98%"),
            ("destructive", "0 62.8% 30.6%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "210 3.7% 15.9%"),
            ("input", "210 3.7% 15.9%"),
            ("ring", "210 4.9% 83.9%"),
        ]),
    }
}

fn taupe() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("background", "0 0% 100%"),
            ("foreground", "30 10% 3.9%"),
            ("card", "0 0% 100%"),
            ("card-foreground", "30 10% 3.9%"),
            ("popover", "0 0% 100%"),
            ("popover-foreground", "30 10% 3.9%"),
            ("primary", "30 5.9% 10%"),
            ("primary-foreground", "0 0% 98%"),
            ("secondary", "30 4.8% 95.9%"),
            ("secondary-foreground", "30 5.9% 10%"),
            ("muted", "30 4.8% 95.9%"),
            ("muted-foreground", "30 3.8% 46.1%"),
            ("accent", "30 4.8% 95.9%"),
            ("accent-foreground", "30 5.9% 10%"),
            ("destructive", "0 84.2% 60.2%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "30 5.9% 90%"),
            ("input", "30 5.9% 90%"),
            ("ring", "30 5.9% 10%"),
        ]),
        dark: HashMap::from([
            ("background", "30 10% 3.9%"),
            ("foreground", "0 0% 98%"),
            ("card", "30 10% 3.9%"),
            ("card-foreground", "0 0% 98%"),
            ("popover", "30 10% 3.9%"),
            ("popover-foreground", "0 0% 98%"),
            ("primary", "0 0% 98%"),
            ("primary-foreground", "30 5.9% 10%"),
            ("secondary", "30 3.7% 15.9%"),
            ("secondary-foreground", "0 0% 98%"),
            ("muted", "30 3.7% 15.9%"),
            ("muted-foreground", "30 5% 64.9%"),
            ("accent", "30 3.7% 15.9%"),
            ("accent-foreground", "0 0% 98%"),
            ("destructive", "0 62.8% 30.6%"),
            ("destructive-foreground", "0 0% 98%"),
            ("border", "30 3.7% 15.9%"),
            ("input", "30 3.7% 15.9%"),
            ("ring", "30 4.9% 83.9%"),
        ]),
    }
}

fn accent_blue() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "221.2 83.2% 53.3%"),
            ("primary-foreground", "210 40% 98%"),
            ("chart-1", "221.2 83.2% 53.3%"),
            ("chart-2", "212 95% 68%"),
            ("chart-3", "216 92% 60%"),
            ("chart-4", "210 98% 78%"),
            ("chart-5", "212 97% 87%"),
        ]),
        dark: HashMap::from([
            ("primary", "217.2 91.2% 59.8%"),
            ("primary-foreground", "222.2 47.4% 11.2%"),
            ("chart-1", "221.2 83.2% 53.3%"),
            ("chart-2", "212 95% 68%"),
            ("chart-3", "216 92% 60%"),
            ("chart-4", "210 98% 78%"),
            ("chart-5", "212 97% 87%"),
        ]),
    }
}

fn accent_green() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "142.1 76.2% 36.3%"),
            ("primary-foreground", "355.7 100% 97.3%"),
            ("chart-1", "142.1 76.2% 36.3%"),
            ("chart-2", "139 65% 50%"),
            ("chart-3", "140 74% 44%"),
            ("chart-4", "137 72% 66%"),
            ("chart-5", "141 75% 82%"),
        ]),
        dark: HashMap::from([
            ("primary", "142.1 70.6% 45.3%"),
            ("primary-foreground", "144.9 80.4% 10%"),
            ("chart-1", "142.1 76.2% 36.3%"),
            ("chart-2", "139 65% 50%"),
            ("chart-3", "140 74% 44%"),
            ("chart-4", "137 72% 66%"),
            ("chart-5", "141 75% 82%"),
        ]),
    }
}

fn accent_red() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "0 72.2% 50.6%"),
            ("primary-foreground", "0 0% 98%"),
            ("chart-1", "0 72.2% 50.6%"),
            ("chart-2", "0 74% 62%"),
            ("chart-3", "0 65% 70%"),
            ("chart-4", "0 77% 80%"),
            ("chart-5", "0 75% 88%"),
        ]),
        dark: HashMap::from([
            ("primary", "0 72.2% 50.6%"),
            ("primary-foreground", "0 85.7% 97.3%"),
            ("chart-1", "0 72.2% 50.6%"),
            ("chart-2", "0 74% 62%"),
            ("chart-3", "0 65% 70%"),
            ("chart-4", "0 77% 80%"),
            ("chart-5", "0 75% 88%"),
        ]),
    }
}

fn accent_amber() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "38 92% 50%"),
            ("primary-foreground", "48 96% 89%"),
            ("chart-1", "38 92% 50%"),
            ("chart-2", "35 90% 62%"),
            ("chart-3", "40 85% 72%"),
            ("chart-4", "36 88% 82%"),
            ("chart-5", "42 87% 90%"),
        ]),
        dark: HashMap::from([
            ("primary", "38 92% 50%"),
            ("primary-foreground", "48 96% 89%"),
            ("chart-1", "38 92% 50%"),
            ("chart-2", "35 90% 62%"),
            ("chart-3", "40 85% 72%"),
            ("chart-4", "36 88% 82%"),
            ("chart-5", "42 87% 90%"),
        ]),
    }
}

fn accent_cyan() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "187 85% 38%"),
            ("primary-foreground", "185 86% 97%"),
            ("chart-1", "187 85% 38%"),
            ("chart-2", "188 80% 50%"),
            ("chart-3", "186 78% 62%"),
            ("chart-4", "189 82% 74%"),
            ("chart-5", "187 80% 86%"),
        ]),
        dark: HashMap::from([
            ("primary", "187 85% 38%"),
            ("primary-foreground", "185 86% 97%"),
            ("chart-1", "187 85% 38%"),
            ("chart-2", "188 80% 50%"),
            ("chart-3", "186 78% 62%"),
            ("chart-4", "189 82% 74%"),
            ("chart-5", "187 80% 86%"),
        ]),
    }
}

fn accent_emerald() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "160 84% 39%"),
            ("primary-foreground", "155 100% 97%"),
            ("chart-1", "160 84% 39%"),
            ("chart-2", "158 78% 52%"),
            ("chart-3", "162 75% 64%"),
            ("chart-4", "157 80% 76%"),
            ("chart-5", "163 78% 88%"),
        ]),
        dark: HashMap::from([
            ("primary", "160 84% 39%"),
            ("primary-foreground", "155 100% 97%"),
            ("chart-1", "160 84% 39%"),
            ("chart-2", "158 78% 52%"),
            ("chart-3", "162 75% 64%"),
            ("chart-4", "157 80% 76%"),
            ("chart-5", "163 78% 88%"),
        ]),
    }
}

fn accent_indigo() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "238 83% 56%"),
            ("primary-foreground", "226 100% 97%"),
            ("chart-1", "238 83% 56%"),
            ("chart-2", "239 80% 66%"),
            ("chart-3", "237 78% 76%"),
            ("chart-4", "240 82% 84%"),
            ("chart-5", "239 80% 92%"),
        ]),
        dark: HashMap::from([
            ("primary", "238 83% 56%"),
            ("primary-foreground", "226 100% 97%"),
            ("chart-1", "238 83% 56%"),
            ("chart-2", "239 80% 66%"),
            ("chart-3", "237 78% 76%"),
            ("chart-4", "240 82% 84%"),
            ("chart-5", "239 80% 92%"),
        ]),
    }
}

fn accent_lime() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "100 70% 40%"),
            ("primary-foreground", "100 100% 95%"),
            ("chart-1", "100 70% 40%"),
            ("chart-2", "102 65% 52%"),
            ("chart-3", "99 62% 64%"),
            ("chart-4", "103 68% 76%"),
            ("chart-5", "100 66% 88%"),
        ]),
        dark: HashMap::from([
            ("primary", "100 70% 40%"),
            ("primary-foreground", "100 100% 95%"),
            ("chart-1", "100 70% 40%"),
            ("chart-2", "102 65% 52%"),
            ("chart-3", "99 62% 64%"),
            ("chart-4", "103 68% 76%"),
            ("chart-5", "100 66% 88%"),
        ]),
    }
}

fn accent_orange() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "24 94% 50%"),
            ("primary-foreground", "30 100% 95%"),
            ("chart-1", "24 94% 50%"),
            ("chart-2", "22 90% 62%"),
            ("chart-3", "26 85% 72%"),
            ("chart-4", "24 88% 82%"),
            ("chart-5", "28 86% 90%"),
        ]),
        dark: HashMap::from([
            ("primary", "24 94% 50%"),
            ("primary-foreground", "30 100% 95%"),
            ("chart-1", "24 94% 50%"),
            ("chart-2", "22 90% 62%"),
            ("chart-3", "26 85% 72%"),
            ("chart-4", "24 88% 82%"),
            ("chart-5", "28 86% 90%"),
        ]),
    }
}

fn accent_pink() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "330 80% 50%"),
            ("primary-foreground", "330 100% 97%"),
            ("chart-1", "330 80% 50%"),
            ("chart-2", "328 75% 62%"),
            ("chart-3", "332 72% 74%"),
            ("chart-4", "329 78% 84%"),
            ("chart-5", "331 76% 92%"),
        ]),
        dark: HashMap::from([
            ("primary", "330 80% 50%"),
            ("primary-foreground", "330 100% 97%"),
            ("chart-1", "330 80% 50%"),
            ("chart-2", "328 75% 62%"),
            ("chart-3", "332 72% 74%"),
            ("chart-4", "329 78% 84%"),
            ("chart-5", "331 76% 92%"),
        ]),
    }
}

fn accent_purple() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "270 80% 56%"),
            ("primary-foreground", "270 100% 97%"),
            ("chart-1", "270 80% 56%"),
            ("chart-2", "272 75% 66%"),
            ("chart-3", "268 72% 76%"),
            ("chart-4", "271 78% 84%"),
            ("chart-5", "270 76% 92%"),
        ]),
        dark: HashMap::from([
            ("primary", "270 80% 56%"),
            ("primary-foreground", "270 100% 97%"),
            ("chart-1", "270 80% 56%"),
            ("chart-2", "272 75% 66%"),
            ("chart-3", "268 72% 76%"),
            ("chart-4", "271 78% 84%"),
            ("chart-5", "270 76% 92%"),
        ]),
    }
}

fn accent_rose() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "346 80% 50%"),
            ("primary-foreground", "346 100% 97%"),
            ("chart-1", "346 80% 50%"),
            ("chart-2", "344 75% 62%"),
            ("chart-3", "348 72% 74%"),
            ("chart-4", "345 78% 84%"),
            ("chart-5", "347 76% 92%"),
        ]),
        dark: HashMap::from([
            ("primary", "346 80% 50%"),
            ("primary-foreground", "346 100% 97%"),
            ("chart-1", "346 80% 50%"),
            ("chart-2", "344 75% 62%"),
            ("chart-3", "348 72% 74%"),
            ("chart-4", "345 78% 84%"),
            ("chart-5", "347 76% 92%"),
        ]),
    }
}

fn accent_sky() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "198 93% 50%"),
            ("primary-foreground", "200 100% 97%"),
            ("chart-1", "198 93% 50%"),
            ("chart-2", "196 88% 60%"),
            ("chart-3", "200 85% 72%"),
            ("chart-4", "197 90% 82%"),
            ("chart-5", "199 88% 90%"),
        ]),
        dark: HashMap::from([
            ("primary", "198 93% 50%"),
            ("primary-foreground", "200 100% 97%"),
            ("chart-1", "198 93% 50%"),
            ("chart-2", "196 88% 60%"),
            ("chart-3", "200 85% 72%"),
            ("chart-4", "197 90% 82%"),
            ("chart-5", "199 88% 90%"),
        ]),
    }
}

fn accent_teal() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "173 80% 36%"),
            ("primary-foreground", "166 100% 97%"),
            ("chart-1", "173 80% 36%"),
            ("chart-2", "175 75% 48%"),
            ("chart-3", "172 72% 60%"),
            ("chart-4", "174 78% 74%"),
            ("chart-5", "173 76% 86%"),
        ]),
        dark: HashMap::from([
            ("primary", "173 80% 36%"),
            ("primary-foreground", "166 100% 97%"),
            ("chart-1", "173 80% 36%"),
            ("chart-2", "175 75% 48%"),
            ("chart-3", "172 72% 60%"),
            ("chart-4", "174 78% 74%"),
            ("chart-5", "173 76% 86%"),
        ]),
    }
}

fn accent_violet() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "256 80% 56%"),
            ("primary-foreground", "256 100% 97%"),
            ("chart-1", "256 80% 56%"),
            ("chart-2", "258 75% 66%"),
            ("chart-3", "254 72% 76%"),
            ("chart-4", "257 78% 84%"),
            ("chart-5", "256 76% 92%"),
        ]),
        dark: HashMap::from([
            ("primary", "256 80% 56%"),
            ("primary-foreground", "256 100% 97%"),
            ("chart-1", "256 80% 56%"),
            ("chart-2", "258 75% 66%"),
            ("chart-3", "254 72% 76%"),
            ("chart-4", "257 78% 84%"),
            ("chart-5", "256 76% 92%"),
        ]),
    }
}

fn accent_yellow() -> ThemeColors {
    ThemeColors {
        light: HashMap::from([
            ("primary", "47 95% 50%"),
            ("primary-foreground", "50 100% 90%"),
            ("chart-1", "47 95% 50%"),
            ("chart-2", "45 90% 60%"),
            ("chart-3", "49 85% 70%"),
            ("chart-4", "46 88% 80%"),
            ("chart-5", "48 86% 90%"),
        ]),
        dark: HashMap::from([
            ("primary", "47 95% 50%"),
            ("primary-foreground", "50 100% 90%"),
            ("chart-1", "47 95% 50%"),
            ("chart-2", "45 90% 60%"),
            ("chart-3", "49 85% 70%"),
            ("chart-4", "46 88% 80%"),
            ("chart-5", "48 86% 90%"),
        ]),
    }
}