# montrs-icons Invariants

## What It Enforces
- All icons are derived from the Lucide icon set, wrapped in MontRS naming conventions
- Icons are grouped into 42 category features for selective compilation
- The `Glyph` enum is the single source of truth for all icon identifiers
- Per-icon convenience components (e.g. `SearchIcon`) are auto-generated in `registry.rs`

## Rules
- Always use `Glyph::find()` for search, not manual string matching
- Always use `Glyph::by_name()` for lookup, not `FromStr` directly
- Use `Icon` component for dynamic/reactive icons, per-icon components for static usage
- Never add new icons by hand — regenerate from upstream Lucide data
- Categories use `strum` props for metadata — never hardcode categories in code

## Boundary
- **In-Scope**: Icon rendering, search, categories, metadata, per-icon components
- **Out-of-Scope**: Icon animation, icon picker UI, icon design tools