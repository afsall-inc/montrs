# The Task Graph

MontRS tasks form a **directed dependency graph**. Running a task runs its
transitive dependencies first, level by level, with each level executed
concurrently under a bounded semaphore.

---

## 1. Declaring dependencies

```toml
[tasks]
build = "cargo build"
test  = { command = "cargo test", depends = ["build"] }
lint  = "cargo clippy -- -D warnings"
ship  = { command = "montrs build", depends = ["test", "lint"] }
```

`montrs run ship` executes `build` and `lint` first, then `test`, then `ship`.
Dependencies were previously parsed but, before this work, were never actually
executed — running `ship` ran only `ship`. That is fixed.

---

## 2. Ordering and parallelism

- The graph is topologically sorted into **levels** using Kahn's algorithm.
- A task is scheduled once every dependency in the level below has completed.
- Ties within a level are sorted by name for deterministic output.
- Each level runs concurrently, gated by a semaphore (default 4 permits).

A cycle leaves its members unscheduled (and the builder reports it) rather than
deadlocking.

---

## 3. Caching task results

Task caching is **opt-in** and requires declared outputs:

```toml
[tasks.assets]
command = "node-less-asset-copy"
sources = ["assets/**"]          # input globs (repo-relative)
outputs = ["target/site/**"]     # what the task produces
cache = { enabled = true, env = ["NODE_ENV"] }
```

With this, the task's key is the content hash of:

- the command (and its args/env),
- the `sources` globs,
- the environment variables listed in `cache.env`,
- any `cache.command_inputs`,
- declared tool versions,
- the hashes of its dependency tasks.

If every declared output still matches its recorded fingerprint, the task is
skipped with `cache hit (up to date)`.

> A task whose `outputs` are left unset (`Auto`) is **never** cached, because its
> side effects cannot be verified. Set `outputs = []` to explicitly cache a task
> with no outputs.

Force a run with `--force`, or disable caching entirely with `--no-cache`
(or `MONTRS_NO_CACHE=1`).

---

## 4. `--affected`: run only what changed

```bash
montrs run test --affected
montrs run test --affected --since origin/main
```

`--affected` uses git change detection (`git diff` + `git status`, no daemon) to
collect changed files, then runs only the tasks whose `sources` globs match. A
task with no `sources` always runs.

---

## 5. The cache under the hood

Both the build pipeline and the task runner use `montrs-cache`, the same
content-addressed store described in
[Incremental Builds](incremental-builds.md). The store is local at
`.montrs/cache/` and is gitignored.
