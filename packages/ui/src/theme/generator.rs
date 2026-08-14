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

use crate::theme::colors::{self};

/// Generates a full `style/main.css` with HSL CSS variables.
pub fn generate_css(
    base_color: &str,
    accent_color: Option<&str>,
    radius: Option<&str>,
) -> String {
    let base = colors::base_colors();
    let base_theme = base.get(base_color).expect("unknown base color");

    let theme = match accent_color {
        Some(accent) => {
            let accents = colors::accent_colors();
            let accent_theme =
                accents.get(accent).expect("unknown accent color");
            colors::merge_accent(base_theme, accent_theme)
        }
        None => (*base_theme).clone(),
    };

    let vars = colors::theme_to_css(&theme, radius);
    format!(
        r#"@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {{
    {}
}}

@layer base {{
    * {{
        @apply border-border;
    }}

    body {{
        @apply bg-background text-foreground;
        font-feature-settings: "rlig" 1, "calt" 1;
    }}
}}

html {{
    scroll-behavior: smooth;
}}

@layer base {{
    *:focus-visible {{
        @apply outline-none ring-2 ring-ring ring-offset-2 ring-offset-background;
    }}
}}

::selection {{
    @apply bg-primary text-primary-foreground;
}}
"#,
        vars
    )
}

/// Generates a `tailwind.toml` with theme.extend.colors mapping to CSS vars.
pub fn generate_tailwind_toml(content_paths: &[&str]) -> String {
    let content = content_paths
        .iter()
        .map(|p| format!("    {:?},", p))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"# Tailwind CSS Configuration for MontRS
# This file is automatically converted to tailwind.config.js by montrs

content = [
{}
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
"#,
        content
    )
}
