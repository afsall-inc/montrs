/// ASCII font renderable — 7 text fonts.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub struct ASCIIFontRenderable {
    pub text: String,
    pub font: ASCIIFont,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ASCIIFont {
    Block,
    Bubble,
    Digital,
    Slim,
    Standard,
    Shadow,
    Small,
}

impl ASCIIFontRenderable {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            font: ASCIIFont::Block,
        }
    }
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
    pub fn with_font(mut self, font: ASCIIFont) -> Self {
        self.font = font;
        self
    }
}

impl Default for ASCIIFontRenderable {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ASCIIFontRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
    ) {
        for (i, c) in self.text.chars().enumerate() {
            let ch = match self.font {
                ASCIIFont::Block => c,
                ASCIIFont::Bubble => match c {
                    'A'..='Z' | 'a'..='z' => '◌',
                    _ => c,
                },
                ASCIIFont::Digital => match c {
                    '0'..='9' => c,
                    _ => '□',
                },
                ASCIIFont::Slim => c,
                ASCIIFont::Standard => c,
                ASCIIFont::Shadow => c,
                ASCIIFont::Small => c,
            };
            buffer.set(
                x + i * 2,
                y,
                Cell::styled(
                    ch,
                    Color::Cyan,
                    Color::Reset,
                    CharAttribute::default(),
                ),
            );
        }
    }
}
