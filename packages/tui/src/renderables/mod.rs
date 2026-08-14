/// Renderable components for the TUI.
pub mod ascii_font;
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
pub mod text_buffer;
pub mod text_renderable;
pub mod textarea;

use crate::buffer::Buffer;
pub use ascii_font::{ASCIIFont, ASCIIFontRenderable};
pub use box_renderable::{BorderStyle, BoxRenderable};
pub use code::CodeRenderable;
pub use diff::{DiffLine, DiffLineKind, DiffMode, DiffRenderable};
pub use editor::EditorRenderable;
pub use frame_buffer::FrameBufferRenderable;
pub use image::{ImageMode, ImageRenderable};
pub use input::InputRenderable;
pub use line_number::LineNumberRenderable;
pub use markdown::MarkdownRenderable;
pub use scrollbar::ScrollBarRenderable;
pub use scrollbox::ScrollBoxRenderable;
pub use select::SelectRenderable;
pub use slider::SliderRenderable;
pub use tab_select::TabSelectRenderable;
pub use table::TextTableRenderable;
pub use text_buffer::TextBufferRenderable;
pub use text_renderable::TextRenderable;
pub use textarea::TextareaRenderable;

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
