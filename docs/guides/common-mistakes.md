# Common Mistakes in MontRS (Router & CLI Edition)

This guide outlines the most frequent architectural and operational pitfalls encountered by developers and agents building with MontRS. Following these guidelines ensures your application remains deterministic, testable, and agent-readable.

---

## 1. Why These Mistakes Are Common

MontRS introduces a paradigm shift that can be counter-intuitive for those coming from other ecosystems:
- **The "React/MVC" Hangover**: Most users come from React, MVC, or backend-only Rust. They expect components to own data fetching or routes to act as controllers.
- **Routes are Boundaries, Not Controllers**: In MontRS, a route is a resource boundary (Loader/Action), not a place to write business logic.
- **The CLI is the Source of Truth**: The CLI-driven structure (Plates and File-based routing) can feel restrictive at first because it prioritizes predictability over "clever" manual wiring.

---

## 2. Routing Mistakes (Highest Priority)

### ❌ Treating Loaders as Services
- **What people do**: Writing complex business logic, database queries, or third-party API calls directly inside a `Loader`.
- **Why it feels reasonable**: It's faster to write everything in one place.
- **Why it breaks MontRS**: It bypasses the `Plate` boundary, making logic hard to reuse and nearly impossible to mock correctly in `TestRuntime`.
- **Correct Approach**: Keep Loaders thin. Call a method on a service injected into a `Plate`.

### ❌ Direct Database Access in Loaders
- **What people do**: Hardcoding database connections or raw `sqlx` calls inside every loader.
- **Why it feels reasonable**: "I just need one quick query."
- **Why it breaks MontRS**: Bypasses connection pooling and backend abstraction provided by the framework.
- **Correct Approach**: Use the `ctx.db()` provided by the framework within a `Plate` service.

### ❌ Mutating State in Loaders
- **What people do**: Updating a database record or changing a `Signal` inside a `load()` function.
- **Why it feels reasonable**: "I just need to increment a view counter while I fetch the post."
- **Why it breaks MontRS**: Loaders are meant to be pure read operations. Mutating state in a loader causes unpredictable side effects during SSR and hydration.
- **Correct Approach**: Use an `Action` for all mutations.

### ❌ Fetching Inside Components
- **What people do**: Using `tokio::spawn` or raw HTTP clients inside a Leptos component.
- **Why it feels reasonable**: Standard practice in many SPA frameworks.
- **Why it breaks MontRS**: It hides data requirements from the Router and Agent. The application spec (`agent.json`) will be incomplete, and SSR will fail to pre-fetch the data.
- **Correct Approach**: Always use a `Loader`. Let the Router provide the data to your view.

### ❌ Skipping Validator Validation
- **What people do**: Using raw `Value` or unvalidated structs for API inputs.
- **Why it feels reasonable**: "The input is simple, I don't need a full validator."
- **Why it breaks MontRS**: Breaks data integrity and deprives agents of the metadata needed to understand valid inputs.
- **Correct Approach**: Always use `#[derive(Validator)]` for all input and output types.

### ❌ Over-Flattening Routes
- **What people do**: Putting all route logic into a single large file or avoiding nested directories.
- **Why it feels reasonable**: Avoids "folder sprawl."
- **Why it breaks MontRS**: MontRS uses file-based routing to build a static route graph. Flattening removes the hierarchy needed for nested layouts and inherited context.
- **Correct Approach**: Follow the file-based hierarchy (`routes/users/[id].rs`).

### ❌ Avoiding Nested Layouts
- **What people do**: Re-implementing sidebars or headers in every single route view.
- **Why it feels reasonable**: "I want full control over every pixel in this specific view."
- **Why it breaks MontRS**: Increases boilerplate and breaks the "intent" of the route tree.
- **Correct Approach**: Define a `layout()` in a parent `mod.rs` to wrap all children.

---

## 3. CLI & Structure Mistakes

### ❌ Ignoring CLI Scaffolding
- **What people do**: Manually creating files and directories instead of using `montrs new` or `montrs generate`.
- **Why it feels reasonable**: "I know exactly where I want my files."
- **Why it breaks MontRS**: You might miss critical boilerplate that the CLI handles for agent-readiness.
- **Correct Approach**: Start with `montrs` commands and modify from there.

### ❌ Mixing Business Logic in Main
- **What people do**: Putting all route registrations and business logic in `main.rs`.
- **Why it feels reasonable**: "It's a small app, I don't need multiple files yet."
- **Why it breaks MontRS**: Violates modularity and makes the app hard to scale or test.
- **Correct Approach**: Use `Plate` implementations to keep your application organized and composable.

### ❌ Renaming Folders Arbitrarily
- **What people do**: Renaming `plates/` to `modules/` or `src/` to `code/`.
- **Why it feels reasonable**: Personal preference or legacy project alignment.
- **Why it breaks MontRS**: The discovery engine expects a specific structure. Breaking it makes your app invisible to the spec generator.
- **Correct Approach**: Stick to the standard MontRS directory structure.

### ❌ Manual Spec/Snapshot Edits
- **What people do**: Trying to manually edit `.agent/agent.json`.
- **Why it feels reasonable**: "I just need to fix one description quickly."
- **Why it breaks MontRS**: The file is auto-generated; manual changes will be overwritten.
- **Correct Approach**: Update your source code (traits/comments) and run `montrs spec`.

---

## 4. State & Reactivity Mistakes

### ❌ Global Signals
- **What people do**: Declaring `static` signals or global mutable state.
- **Why it feels reasonable**: Easiest way to share state across the entire app.
- **Why it breaks MontRS**: Breaks **State Locality**. It makes the app non-deterministic and causes race conditions during concurrent SSR requests.
- **Correct Approach**: Keep state in `Plates` or pass `Signals` through context.

### ❌ Hidden Mutation
- **What people do**: Changing a value deep inside a struct without going through an `Action`.
- **Why it feels reasonable**: "It's just a small internal flag."
- **Why it breaks MontRS**: Mutations must be explicit to be auditable by the framework and agent.
- **Correct Approach**: Use an `Action` or a `Signal::set()` call.

---

## 5. Plate Misuse (Router Interaction)

### ❌ Calling Routes from Plates
- **What people do**: Trying to invoke a `Loader` directly from a `Plate` service.
- **Why it feels reasonable**: "I need the same data the UI needs."
- **Why it breaks MontRS**: Creates circular dependencies. Loaders depend on Plates, not the other way around.
- **Correct Approach**: Extract the shared logic into a separate service method within the Plate.

### ❌ Skipping Plates for "Small" Side Effects
- **What people do**: Putting raw `sqlx` calls directly in a Route Action.
- **Why it feels reasonable**: "It's just one query."
- **Why it breaks MontRS**: Violates the **Side Effect Isolation** invariant and makes the route untestable.
- **Correct Approach**: Always wrap side effects (DB, Network, Disk) in a `Plate`.

---

## 6. Testing & Debugging Mistakes

### ❌ Testing UI Instead of Behavior
- **What people do**: Writing 100% Playwright E2E tests for basic business logic.
- **Why it feels reasonable**: "It's the only way to be sure it actually works."
- **Why it breaks MontRS**: E2E tests are slow and flaky. MontRS is designed for deterministic logic testing.
- **Correct Approach**: Test the `Loader` and `Action` logic using `TestRuntime`.

### ❌ Avoiding TestRuntime / Non-Deterministic Tests
- **What people do**: Relying on global state, system time, or live production databases in tests.
- **Why it feels reasonable**: "I want to test with real data."
- **Why it breaks MontRS**: Tests become non-deterministic and flaky.
- **Correct Approach**: Use `montrs-test` fixtures and `TestRuntime` to mock the environment.

---

## 7. "This Is Intentional" (Router Edition)

MontRS makes specific trade-offs to ensure machine-readability:
- **Explicit Loaders/Actions**: More typing than a "hook," but 100% discoverable by an agent.
- **No Magic Data Propagation**: Data flows through the `Context`. If it's not there, your logic can't see it.
- **Strict Boundaries**: Separation between `Plate` and `Router` allows one codebase to run on Web, Mobile, and Desktop.

---

## 8. Recovery Guide

If you've "fought the framework":
1.  **Run `montrs spec`**: See what the framework currently "sees."
2.  **Move Logic to Plates**: Extract DB/Network calls from routes.
3.  **Thin Your Loaders**: Loaders should only parse input and call a Plate.
4.  **Align with CLI**: Use `montrs generate` to see the idiomatic pattern for new features.

---

## 9. Agent-Specific Guidance (For AI Coding Partners)

When working on a MontRS project, AI agents MUST:
- **Do Not Invent APIs**: Stick to the traits defined in `montrs-core`.
- **Do Not Bypass the CLI**: Think "How would the CLI scaffold this?"
- **Provide Agent Metadata**: Never leave `description()` as `None`. It is the primary way other agents (and humans) understand your code's intent.
- **Avoid Complex Macro Logic**: Do not hide business logic inside procedural macros. Prefer explicit traits and functions.
- **Check `.agent/agent.json`**: Ensure your changes are correctly reflected in the project specification.
