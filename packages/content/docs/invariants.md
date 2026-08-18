# Content Package Invariants

## Responsibility
`montrs-content` loads typed Markdown collections for documentation, static pages, and application content.

## Invariants
- File discovery and entry ordering are deterministic.
- Frontmatter is deserialized into caller-defined types.
- Malformed content returns structured errors.
- Rendering does not claim to sanitize untrusted HTML; callers must sanitize before injection.
- The package remains independent of Leptos and the browser.
