# Agent Guide: montrs-log

## Core Concepts
Structured log store with configurable retention, streaming, and rotation.

### Log Records
- Each log entry has a timestamp, level, target, and structured message.
- Levels: `Error`, `Warn`, `Info`, `Debug`, `Trace`.
- Entries are stored in a structured format (JSON by default).

### Retention
- Retention policies are configurable by time (e.g., 7 days) or size (e.g., 100 MB).
- Old logs are automatically pruned based on the active policy.
- Archive strategies can be configured for long-term storage.

### Streaming
- Logs can be streamed in real-time via `LogStream`.
- Use `subscribe()` to receive new log entries as they arrive.
- Filters can be applied at the stream level (by level, target, or keyword).

### Rotation
- Log files are rotated when they reach a configured size threshold.
- Rotation preserves a configurable number of historical files.
- Compression options are available for rotated files.

## Important Rules
- Logs are stored with structured metadata for agent consumption.
- Retention policies are configurable — never hardcode.
- Rotation is automatic based on size or time configuration.
- Streaming is pull-based; subscribers must manage their own backpressure.