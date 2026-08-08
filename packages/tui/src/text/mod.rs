pub mod edit_buffer;
/// Text buffer and styled text.
pub mod rope;
pub mod styled_text;

/// A chunk of styled text.
#[derive(Debug, Clone)]
pub struct StyledChunk {
    pub text: String,
    pub style: CharStyle,
}

#[derive(Debug, Clone, Copy)]
pub struct CharStyle {
    pub fg: crate::buffer::Color,
    pub bg: crate::buffer::Color,
    pub bold: bool,
}

impl Default for CharStyle {
    fn default() -> Self {
        Self {
            fg: crate::buffer::Color::Reset,
            bg: crate::buffer::Color::Reset,
            bold: false,
        }
    }
}
