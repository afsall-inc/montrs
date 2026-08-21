# Agent Guide: montrs-table-core

## Core Concepts
Headless, serializable table state and row models.

### ColumnDef
- `ColumnDef::new(id, header)` — define a column with stable ID.
- `.accessor(fn)` — value accessor function.
- `.sorting(fn)` — custom sorting comparator.
- `.filtering(fn)` — custom filter predicate.
- `.size(u32)` / `.min_size(u32)` / `.max_size(u32)` — column sizing.
- Builder methods: `.disable_sorting()`, `.disable_filtering()`, `.disable_pinning()`, `.disable_resizing()`.

### TableState
- `sorting`, `global_filter`, `column_filters`, `pagination`, `selected_rows`, `hidden_columns`, `column_order`, `expanded_rows`, `pinned_rows_top/bottom`, `pinned_columns_left/right`, `column_widths`, `selected_cells`.

### RowModel Pipeline
- `core_rows` → `filter_rows()` → `sort_rows()` → `paginate()`.
- `rebuild(columns, state)` — runs the full pipeline.

### Table
- `sort_by(column, direction, compare)` — single-column sort.
- `select_row(id)` / `deselect_row(id)` / `clear_selection()`.
- `toggle_row(id)` / `expand_row(id)` / `collapse_row(id)`.
- `select_cell(row_id, column)` / `deselect_cell(row_id, column)`.

### ServerSideMode
- `manual_sorting`, `manual_filtering`, `manual_pagination` — flags for manual mode.

## Important Rules
- No Leptos, DOM, or renderer dependency.
- Row selection uses stable row IDs, never array indexes.
- Table state is serializable.
- Column IDs are stable and independent of physical field order.