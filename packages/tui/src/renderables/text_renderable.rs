/// Text renderable — draws styled text.
use crate::buffer::{Buffer, Cell, CharAttribute, Color};
use crate::renderables::Renderable;

pub fn write_text(
    buffer: &mut Buffer,
    x: usize,
    y: usize,
    text: &str,
    fg: Color,
    bg: Color,
    attr: CharAttribute,
) {
    let mut cx = x;
    for ch in text.chars() {
        if cx >= buffer.width {
            break;
        }
        buffer.set(cx, y, Cell::styled(ch, fg, bg, attr));
        cx += 1;
    }
}

pub struct TextRenderable {
    pub text: String,
    pub fg: Color,
    pub bg: Color,
    pub attr: CharAttribute,
}

impl TextRenderable {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            attr: CharAttribute::default(),
        }
    }
}

impl Renderable for TextRenderable {
    fn render(
        &self,
        buffer: &mut Buffer,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
    ) {
        write_text(buffer, x, y, &self.text, self.fg, self.bg, self.attr);
    }
}
