# montrs-log — Invariants

- **Layer**: 2 (no deps on other montrs packages)
- **Storage**: File-based, one `.log` file per service
- **Formats**: Text, JSON, logfmt
- **Retention**: Line-count-based (configurable max_lines), age-based (max_age_secs), archive
- **Streaming**: SSE via `tail()` for live log viewing
- **Querying**: Filter by service name, level, limit, offset
- **Safe file names**: Non-alphanumeric chars replaced with `_`
- **Default root**: `~/.local/state/montrs/logs` or `$MONTRS_STATE/logs`