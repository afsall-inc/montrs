//! Native menu item types for desktop/mobile platform adapters.

/// A single item in a native menu bar or context menu.
#[derive(Debug, Clone)]
pub struct NativeMenuItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub enabled: bool,
    pub children: Vec<NativeMenuItem>,
}
