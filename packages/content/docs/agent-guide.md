# Agent Guide: montrs-content

## Core Concepts
Typed Markdown content collections inspired by Astro's content collections.

### Collection
- `Collection::<T>::load("path")` — runtime filesystem loading (SSR).
- `Collection::from_embedded(entries)` — compile-time embedded content.
- `Collection::from_entries(entries)` — build from a Vec, sorted by slug.

### Entry
- `entry.slug` — URL-friendly identifier derived from the filename.
- `entry.data` — deserialized YAML frontmatter.
- `entry.body` — raw Markdown body.
- `entry.render()` — renders Markdown to HTML.

### Codegen
- `codegen::generate(content_dir, name)` — generates Rust source for build.rs.
- `codegen::generate_to_out_dir(content_dir, name)` — writes to OUT_DIR.
- `Entry::from_embedded(source, slug)` — parses embedded content at runtime.

## Important Rules
- File discovery and entry ordering are deterministic.
- Frontmatter deserializes into caller-defined types.
- Render output is not sanitized; callers must sanitize before injection.
- The package is independent of Leptos and the browser.