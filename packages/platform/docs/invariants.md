# Platform Package Invariants

## 1. Responsibility
`montrs-platform` is the layer-0 platform abstraction. It defines the `Target` enum and `PlatformAdapter` trait that the rest of the framework uses to interact with platform-specific capabilities.

## 2. Invariants
- **Zero MontRS Dependencies**: This package must not depend on any other `montrs-*` package.
- **Trait-Driven**: All platform capabilities must be expressed via the `PlatformAdapter` trait.
- **No-Op Defaults**: Every platform adapter method must have a sensible no-op default so the framework compiles on any target.
- **Target Enum**: The `Target` enum is the single source of truth for execution environment identification.

## 3. Boundary Definitions
- **In-Scope**: `Target` enum, `PlatformAdapter` trait, `NoopPlatformAdapter`, `NativeMenuItem` types.
- **Out-of-Scope**: UI rendering, window creation, event loops, platform-specific shell implementations.

## 4. Agent Guidelines
- When adding a new platform variant to `Target`, update all match arms across the framework.
- New adapter methods should have default no-op implementations to avoid breaking existing adapters.