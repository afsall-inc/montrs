# UI Package Invariants

## 1. Responsibility
`montrs-ui` provides pre-built Leptos components for MontRS applications. Includes buttons, inputs, dialogs, navigation, and layout components.

## 2. Invariants
- **Leptos Components**: All components use Leptos 0.8 signals and the `view!` macro.
- **Casing**: Component names are `PascalCase`, attributes are `kebab-case`.
- **cn! Macro**: The `cn!` macro (tailwind-merge based) is used for class merging.
- **No DOM Direct Access**: Components must use Leptos abstractions, not direct `web-sys` where possible.

## 3. Boundary Definitions
- **In-Scope**: UI components, variants, theme provider, utility hooks.
- **Out-of-Scope**: Icons (separate package), motion (separate package), routing.

## 4. Agent Guidelines
- Follow the `variants!` macro pattern for component styling variants.
- Use `RwSignal` for reactive state.
- Components must be testable in isolation.