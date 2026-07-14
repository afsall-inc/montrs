---
title: "feat(prdoc): extract as standalone reusable package with rich summaries and LLM enhancement"
pr: 79
author: "balqaasem"
status: draft
packages: ["agent", "cli", "prdoc"]
breaking: true
needs-review: ["architecture", "design"]
audience: ["framework_dev", "agent_user", "operator"]
crates:
  - name: agent
    bump: major
    validate: true
  - name: cli
    bump: minor
    validate: true
  - name: prdoc
    bump: minor
    validate: true
---

## Summary

Adds prdoc): extract as standalone reusable package with rich summari…. Introduces new functions scaffold_project, as_str, analyze_diff, analyze_commits, classify_commit, gather_pr_context_from_gh, get_diff_for_pr, get_diff_for_range, get_commit_messages_for_range, as_str, new, add_prdoc, render, determine_next_version, collect_prdocs_from_git, load_prdocs_from_dir, to_llm_config, to_llm_config, load_config, find_project_root, classify_by_embedding, generate_prdoc, render_prdoc, render_prdoc_rich, create_provider, enhance_summary, new, generate_rich_summary, extract_public_api_from_diff, as_str, from_str_lossy, dominates, as_str, parse_prdoc, load_prdoc, validate_prdoc, new structs CounterState, CreateUserInput, UserRoute, ProjectConfig, User, UserRoute, CreateTodoInput, MyBench, FileChange, DiffAnalysis, PrContext, ChangelogEntry, Changelog, PrdocConfig, LlmSection, GenerateSection, LlmConfig, GroqProvider, OpenAiProvider, OllamaProvider, LocalLlmProvider, SummaryContext, PublicApiChange, PrDoc, CrateChange, new enums ChangeCategory, FileCategory, ChangelogCategory, PrDocStatus, BumpLevel, Audience, new traits LlmSummaryProvider. Removes CreateTodoInput, scaffold_project, CounterState, CreateUserInput, MyBench, UserRoute, UserRoute, ProjectConfig, User, ChangelogEntry, ChangelogCategory, as_str, Changelog, new, add_prdoc, render, determine_next_version, collect_prdocs_from_git, load_prdocs_from_dir, PrDoc, PrDocStatus, BumpLevel, as_str, from_str_lossy, dominates, Audience, as_str, CrateChange, parse_prdoc, load_prdoc, validate_prdoc, ChangeCategory, as_str, FileChange, FileCategory, DiffAnalysis, PrContext, analyze_diff, analyze_commits, classify_commit, gather_pr_context_from_gh, get_diff_for_pr, get_diff_for_range, get_commit_messages_for_range, classify_by_embedding, generate_prdoc, render_prdoc. Affects: agent (6 source file(s)) [major] +8/-1441, cli (1 source file(s)) [minor] +6/-2, prdoc (11 source file(s), 1 doc file(s)) [minor] +2806/-0. This change introduces breaking modifications to the public API.

## Changes
### Packages Affected
- **agent** (bump: major): See summary for details.
- **cli** (bump: minor): See summary for details.
- **prdoc** (bump: minor): See summary for details.

## Agent Instructions
### Verification
1. Run `cargo test --workspace` — all tests must pass.
2. Run `cargo clippy --workspace -- -D warnings` — no warnings.
3. Run `montrs agent check` — no invariant violations.

### Review Focus

## Migration Notes

This PR introduces breaking changes. Review the public API modifications carefully.
