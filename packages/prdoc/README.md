# montrs-prdoc

Structured PR documentation, auto-generation, changelog, and SemVer bumping for Rust projects. Usable standalone — zero MontRS framework dependency.

## Install

```bash
# CLI tool (no LLM)
cargo install montrs-prdoc --features cli

# CLI with LLM-enhanced summaries (Groq, OpenAI, Ollama)
cargo install montrs-prdoc --features cli,llm

# CLI with local LLM (candle + Qwen2-0.5B)
cargo install montrs-prdoc --features cli,local-llm
```

## Quick Start (CLI)

```bash
# Auto-generate prdoc.md from git history
prdoc generate --from-commits main..HEAD

# From a PR number (requires gh CLI + GitHub token)
prdoc generate --pr 42

# With LLM-enhanced summary (requires prdoc.toml or env vars)
prdoc generate --pr 42 --llm

# Validate
prdoc validate

# Display as JSON
prdoc show

# Generate changelog
prdoc changelog generate --from v0.1.0

# Compute version bumps
prdoc changelog bump --current 0.1.0 --dry-run

# Verify all PRs have prdocs
prdoc changelog verify --from v0.1.0
```

## Library Usage

```rust
use montrs_prdoc::{parse_prdoc, validate_prdoc};

let content = std::fs::read_to_string("prdoc.md").unwrap();
let prdoc = parse_prdoc(&content).unwrap();
let issues = validate_prdoc(&prdoc);
```

### Auto-Generation

```rust
use montrs_prdoc::{analyze_diff, generate_prdoc, render_prdoc};

let diff = std::fs::read_to_string("changes.diff").unwrap();
let analysis = analyze_diff(&diff);
let prdoc = generate_prdoc(&analysis, None);
let rendered = render_prdoc(&prdoc, &analysis);
std::fs::write("prdoc.md", rendered).unwrap();
```

### Changelog

```rust
use montrs_prdoc::{Changelog, load_prdoc};

let prdoc = load_prdoc(&std::path::Path::new("prdoc.md")).unwrap();
let mut changelog = Changelog::new();
changelog.add_prdoc(&prdoc);
std::fs::write("CHANGELOG.md", changelog.render()).unwrap();
```

## Configuration

Create a `prdoc.toml` in your project root:

```toml
[llm]
provider = "groq"          # groq, openai, ollama, none
model = "llama-3.1-8b-instant"
api_key_env = "GROQ_API_KEY"

[generate]
default_output = "prdoc.md"
```

Or set via environment:
- `PRDOC_LLM_PROVIDER=groq`
- `PRDOC_API_KEY=sk-...`

## Features

| Feature | Description | Dependencies |
|---------|-------------|-------------|
| `default` | Core types, parsing, validation, diff analysis, generator, embed classifier, changelog | serde, serde_yaml, chrono, regex, toml |
| `cli` | Standalone `prdoc` binary | + clap |
| `llm` | LLM-enhanced summaries (Groq, OpenAI, Ollama) | + reqwest |
| `local-llm` | Local LLM inference via candle | + llm, candle-core, candle-nn, candle-transformers, hf-hub |

## CI Integration

```yaml
# .github/workflows/prdoc.yml
- name: Install prdoc
  run: cargo install montrs-prdoc --features cli
- name: Generate PRDoc
  run: prdoc generate --pr ${{ github.event.pull_request.number }} --force
- name: Validate
  run: prdoc validate
```

## License

MIT
