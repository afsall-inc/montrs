# Doc Linting Checklist

To ensure that MontRS documentation remains machine-readable, architecturally sound, and consistent with the framework's principles, use this checklist when creating or updating `.md` files.

## ✅ Checklist Items

- [ ] **Structural Integrity**: Does the doc use hierarchical headings (`#`, `##`, `###`)?
- [ ] **Agent-First Annotations**: Are public framework tools marked with `@agent-tool` in code examples?
- [ ] **Invariant Alignment**: Are new architectural rules reflected in `docs/invariants.md`?
- [ ] **Metadata Completeness**: Do code examples for traits (`Plate`, `Loader`, `Action`) include `description()` implementations?
- [ ] **Link Validation**: Are all internal links (`[text](../../path/to/file.md)`) valid and relative to the current file?
- [ ] **Terminology Consistency**: Does the doc use standard MontRS terms (Plates, Routes, Loaders, Actions, Signals)?
- [ ] **Conciseness & Clarity**: Is the language "visionary yet rigorous"? Avoid fluff and prioritize actionable information.
- [ ] **Machine-Readable Formatting**: Are code blocks correctly labeled with language tags (e.g., ` ```rust `)?
- [ ] **Package Boundaries**: Does the doc respect the package boundaries defined in `docs/architecture/packages.md`?
- [ ] **Error Handling**: Are `AgentError` patterns used in examples for error-prone operations?

---

## 🤖 Why This Matters
MontRS is an **Agent-Native** framework. Our documentation is not just for humans; it is the primary source of truth for the Agents building and maintaining the ecosystem. Following these rules ensures that Agents can accurately navigate, understand, and modify the codebase without introducing architectural drift.
