//! Headless table state, column definitions, and deterministic row transformations.
//! A simple hybrid inspired by TanStack Table: typed column defs + a row-model pipeline.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// ============================================================================
// ColumnId
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColumnId(pub String);

impl ColumnId {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}
impl From<&str> for ColumnId { fn from(value: &str) -> Self { Self(value.to_string()) } }
impl From<String> for ColumnId { fn from(value: String) -> Self { Self(value) } }
impl std::fmt::Display for ColumnId { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }

// ============================================================================
// Sort / Filter / Pagination
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortDirection { Ascending, Descending }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortEntry { pub column: ColumnId, pub direction: SortDirection }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination { pub page_index: usize, pub page_size: usize }
impl Default for Pagination { fn default() -> Self { Self { page_index: 0, page_size: 25 } } }

// ============================================================================
// TableState
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TableState {
    pub sorting: Vec<SortEntry>,
    pub global_filter: Option<String>,
    pub column_filters: Vec<ColumnFilter>,
    pub pagination: Pagination,
    pub selected_rows: Vec<String>,
    pub hidden_columns: Vec<ColumnId>,
    pub column_order: Vec<ColumnId>,
    pub expanded_rows: Vec<String>,
    pub pinned_rows_top: Vec<String>,
    pub pinned_rows_bottom: Vec<String>,
    pub pinned_columns_left: Vec<ColumnId>,
    pub pinned_columns_right: Vec<ColumnId>,
    pub column_widths: Vec<(ColumnId, u32)>,
    pub selected_cells: Vec<(String, ColumnId)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnFilter { pub column: ColumnId, pub value: String }

// ============================================================================
// Row
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row<T> { pub id: String, pub value: T }

// ============================================================================
// ColumnDef
// ============================================================================

pub type SortingFn<T> = fn(&T, &T) -> Ordering;
pub type FilterFn<T> = fn(&T, &str) -> bool;
pub type GroupingFn<T> = fn(&T) -> String;
pub type AccessorFn<T> = fn(&T) -> String;

#[derive(Debug, Clone)]
pub struct ColumnDef<T> {
    pub id: ColumnId,
    pub header: String,
    pub accessor: Option<AccessorFn<T>>,
    pub sorting_fn: Option<SortingFn<T>>,
    pub filtering_fn: Option<FilterFn<T>>,
    pub grouping_fn: Option<GroupingFn<T>>,
    pub size: Option<u32>,
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
    pub enable_sorting: bool,
    pub enable_filtering: bool,
    pub enable_pinning: bool,
    pub enable_resizing: bool,
}

impl<T> ColumnDef<T> {
    pub fn new(id: impl Into<String>, header: impl Into<String>) -> Self {
        Self {
            id: id.into().into(),
            header: header.into(),
            accessor: None,
            sorting_fn: None,
            filtering_fn: None,
            grouping_fn: None,
            size: None,
            min_size: None,
            max_size: None,
            enable_sorting: true,
            enable_filtering: true,
            enable_pinning: true,
            enable_resizing: true,
        }
    }
    pub fn accessor(mut self, f: AccessorFn<T>) -> Self { self.accessor = Some(f); self }
    pub fn sorting(mut self, f: SortingFn<T>) -> Self { self.sorting_fn = Some(f); self }
    pub fn filtering(mut self, f: FilterFn<T>) -> Self { self.filtering_fn = Some(f); self }
    pub fn size(mut self, size: u32) -> Self { self.size = Some(size); self }
    pub fn min_size(mut self, size: u32) -> Self { self.min_size = Some(size); self }
    pub fn max_size(mut self, size: u32) -> Self { self.max_size = Some(size); self }
    pub fn disable_sorting(mut self) -> Self { self.enable_sorting = false; self }
    pub fn disable_filtering(mut self) -> Self { self.enable_filtering = false; self }
    pub fn disable_pinning(mut self) -> Self { self.enable_pinning = false; self }
    pub fn disable_resizing(mut self) -> Self { self.enable_resizing = false; self }
}

// ============================================================================
// Server-side (manual) mode
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ServerSideMode {
    pub manual_sorting: bool,
    pub manual_filtering: bool,
    pub manual_pagination: bool,
}

// ============================================================================
// RowModel pipeline
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct RowModel<T: Clone> {
    core_rows: Vec<Row<T>>,
    filtered_rows: Vec<Row<T>>,
    sorted_rows: Vec<Row<T>>,
    paginated_rows: Vec<Row<T>>,
}

impl<T: Clone> RowModel<T> {
    pub fn new(rows: Vec<Row<T>>) -> Self {
        let filtered = rows.clone();
        let sorted = filtered.clone();
        let paginated = sorted.clone();
        Self { core_rows: rows, filtered_rows: filtered, sorted_rows: sorted, paginated_rows: paginated }
    }

    pub fn core_rows(&self) -> &[Row<T>] { &self.core_rows }
    pub fn rows(&self) -> &[Row<T>] { &self.paginated_rows }

    pub fn filter_rows(&mut self, columns: &[ColumnDef<T>], filters: &[ColumnFilter], global: Option<&str>)
    where T: 'static {
        let mut rows = self.core_rows.clone();
        if let Some(global) = global {
            let g = global.to_ascii_lowercase();
            rows.retain(|row| {
                columns.iter().any(|col| {
                    col.accessor.map(|f| f(&row.value).to_ascii_lowercase().contains(&g)).unwrap_or(false)
                })
            });
        }
        rows.retain(|row| {
            filters.iter().all(|filter| {
                columns.iter().find(|c| c.id == filter.column)
                    .and_then(|c| c.filtering_fn)
                    .map(|f| f(&row.value, &filter.value))
                    .unwrap_or(true)
            })
        });
        self.filtered_rows = rows;
    }

    pub fn sort_rows(&mut self, sorting: &[SortEntry], columns: &[ColumnDef<T>])
    where T: 'static {
        let mut rows = self.filtered_rows.clone();
        rows.sort_by(|left, right| {
            for entry in sorting {
                if let Some(col) = columns.iter().find(|c| c.id == entry.column) {
                    let ordering = col.sorting_fn.map(|f| f(&left.value, &right.value)).unwrap_or(Ordering::Equal);
                    if ordering != Ordering::Equal {
                        return match entry.direction {
                            SortDirection::Ascending => ordering,
                            SortDirection::Descending => ordering.reverse(),
                        };
                    }
                }
            }
            Ordering::Equal
        });
        self.sorted_rows = rows;
    }

    pub fn paginate(&mut self, pagination: &Pagination) {
        let start = pagination.page_index.saturating_mul(pagination.page_size);
        self.paginated_rows = self.sorted_rows.iter().skip(start).take(pagination.page_size).cloned().collect();
    }

    pub fn rebuild(&mut self, columns: &[ColumnDef<T>], state: &TableState)
    where T: 'static {
        self.filter_rows(columns, &state.column_filters, state.global_filter.as_deref());
        self.sort_rows(&state.sorting, columns);
        self.paginate(&state.pagination);
    }
}

// ============================================================================
// Table
// ============================================================================

#[derive(Debug, Clone)]
pub struct Table<T: Clone> {
    rows: Vec<Row<T>>,
    state: TableState,
    model: RowModel<T>,
}

impl<T: Clone + 'static> Table<T> {
    pub fn new(rows: Vec<Row<T>>) -> Self {
        let model = RowModel::new(rows.clone());
        Self { rows, state: TableState::default(), model }
    }
    pub fn state(&self) -> &TableState { &self.state }
    pub fn state_mut(&mut self) -> &mut TableState { &mut self.state }
    pub fn rows(&self) -> &[Row<T>] { &self.rows }
    pub fn model(&self) -> &RowModel<T> { &self.model }
    pub fn model_mut(&mut self) -> &mut RowModel<T> { &mut self.model }
    pub fn visible_rows(&self) -> &[Row<T>] { self.model.rows() }

    pub fn sort_by<F>(&mut self, column: ColumnId, direction: SortDirection, compare: F)
    where F: Fn(&T, &T) -> Ordering {
        self.state.sorting = vec![SortEntry { column, direction }];
        self.rows.sort_by(|left, right| {
            let ordering = compare(&left.value, &right.value);
            match direction { SortDirection::Ascending => ordering, SortDirection::Descending => ordering.reverse() }
        });
        let columns: Vec<ColumnDef<T>> = Vec::new();
        self.model.sort_rows(&self.state.sorting, &columns);
    }

    pub fn select_row(&mut self, id: impl Into<String>) {
        let id = id.into();
        if !self.state.selected_rows.contains(&id) { self.state.selected_rows.push(id); }
    }
    pub fn deselect_row(&mut self, id: &str) { self.state.selected_rows.retain(|s| s != id); }
    pub fn clear_selection(&mut self) { self.state.selected_rows.clear(); }

    pub fn expand_row(&mut self, id: impl Into<String>) {
        let id = id.into();
        if !self.state.expanded_rows.contains(&id) { self.state.expanded_rows.push(id); }
    }
    pub fn collapse_row(&mut self, id: &str) { self.state.expanded_rows.retain(|r| r != id); }
    pub fn toggle_row(&mut self, id: impl Into<String>) {
        let id = id.into();
        if self.state.expanded_rows.contains(&id) { self.state.expanded_rows.retain(|r| r != &id); }
        else { self.state.expanded_rows.push(id); }
    }

    pub fn select_cell(&mut self, row_id: impl Into<String>, column: ColumnId) {
        let cell = (row_id.into(), column);
        if !self.state.selected_cells.contains(&cell) { self.state.selected_cells.push(cell); }
    }
    pub fn deselect_cell(&mut self, row_id: &str, column: &ColumnId) {
        self.state.selected_cells.retain(|(r, c)| r != row_id || c != column);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Person { name: String, age: u32 }

    #[test]
    fn paginates_and_selects_by_stable_id() {
        let mut table: Table<u32> = Table::new((0..3).map(|v| Row { id: v.to_string(), value: v }).collect());
        table.state_mut().pagination.page_size = 2;
        table.select_row("2");
        let pagination = table.state().pagination.clone();
        table.model_mut().paginate(&pagination);
        assert_eq!(table.visible_rows().len(), 2);
        assert_eq!(table.state().selected_rows, vec!["2"]);
    }

    #[test]
    fn column_def_sorting_and_filtering() {
        let columns = vec![
            ColumnDef::<Person>::new("name", "Name").accessor(|p| p.name.clone()),
            ColumnDef::<Person>::new("age", "Age")
                .accessor(|p| p.age.to_string())
                .sorting(|a, b| a.age.cmp(&b.age))
                .filtering(|p, q| p.age.to_string().contains(q)),
        ];
        let rows = vec![
            Row { id: "a".into(), value: Person { name: "Alice".into(), age: 30 } },
            Row { id: "b".into(), value: Person { name: "Bob".into(), age: 25 } },
        ];
        let mut model = RowModel::new(rows);
        model.sort_rows(&[SortEntry { column: "age".into(), direction: SortDirection::Ascending }], &columns);
        assert_eq!(model.core_rows()[0].value.age, 30);
        model.paginate(&Pagination { page_index: 0, page_size: 25 });
        assert_eq!(model.rows().len(), 2);
    }

    #[test]
    fn expansion_toggle_works() {
        let mut table: Table<u32> = Table::new(vec![Row { id: "1".into(), value: 1 }]);
        table.toggle_row("1");
        assert_eq!(table.state().expanded_rows, vec!["1"]);
        table.toggle_row("1");
        assert!(table.state().expanded_rows.is_empty());
    }
}
