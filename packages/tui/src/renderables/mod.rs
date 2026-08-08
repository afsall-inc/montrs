pub mod ascii_font;
/// Renderable components for the TUI.
pub mod box_renderable;
pub mod code;
pub mod diff;
pub mod editor;
pub mod frame_buffer;
pub mod image;
pub mod input;
pub mod line_number;
pub mod markdown;
pub mod scrollbar;
pub mod scrollbox;
pub mod select;
pub mod slider;
pub mod tab_select;
pub mod table;
pub mod text_renderable;
pub mod textarea;

use crate::buffer::Buffer;

/// Base trait for all renderable objects.
pub trait Renderable: Send + Sync {
    /// Render this object into a buffer at the given position and size.
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    );
    /// The minimum width needed.
    fn min_width(&self) -> usize {
        1
    }
    /// The minimum height needed.
    fn min_height(&self) -> usize {
        1
    }
}
