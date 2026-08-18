//! Headless table state and deterministic row transformations.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColumnId(pub String);

impl From<&str> for ColumnId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortEntry {
    pub column: ColumnId,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    pub page_index: usize,
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page_index: 0, page_size: 25 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TableState {
    pub sorting: Vec<SortEntry>,
    pub global_filter: Option<String>,
    pub pagination: Pagination,
    pub selected_rows: Vec<String>,
    pub hidden_columns: Vec<ColumnId>,
    pub column_order: Vec<ColumnId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row<T> {
    pub id: String,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct Table<T> {
    rows: Vec<Row<T>>,
    state: TableState,
}

impl<T> Table<T> {
    pub fn new(rows: Vec<Row<T>>) -> Self {
        Self { rows, state: TableState::default() }
    }

    pub fn state(&self) -> &TableState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut TableState {
        &mut self.state
    }

    pub fn rows(&self) -> &[Row<T>] {
        &self.rows
    }

    pub fn visible_rows(&self) -> Vec<&Row<T>> {
        let start = self.state.pagination.page_index.saturating_mul(self.state.pagination.page_size);
        self.rows
            .iter()
            .skip(start)
            .take(self.state.pagination.page_size)
            .collect()
    }

    pub fn sort_by<F>(&mut self, column: ColumnId, direction: SortDirection, compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        self.state.sorting = vec![SortEntry { column, direction }];
        self.rows.sort_by(|left, right| {
            let ordering = compare(&left.value, &right.value);
            match direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            }
        });
    }

    pub fn select_row(&mut self, id: impl Into<String>) {
        let id = id.into();
        if !self.state.selected_rows.contains(&id) {
            self.state.selected_rows.push(id);
        }
    }

    pub fn deselect_row(&mut self, id: &str) {
        self.state.selected_rows.retain(|selected| selected != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paginates_and_selects_by_stable_id() {
        let mut table = Table::new((0..3).map(|value| Row { id: value.to_string(), value }).collect());
        table.state_mut().pagination.page_size = 2;
        table.select_row("2");
        assert_eq!(table.visible_rows().len(), 2);
        assert_eq!(table.state().selected_rows, vec!["2"]);
    }

    #[test]
    fn sorting_is_explicit() {
        let mut table = Table::new(vec![Row { id: "b".into(), value: 2 }, Row { id: "a".into(), value: 1 }]);
        table.sort_by("value".into(), SortDirection::Ascending, |left, right| left.cmp(right));
        assert_eq!(table.rows()[0].value, 1);
    }
}
