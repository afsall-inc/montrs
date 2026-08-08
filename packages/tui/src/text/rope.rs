/// A rope data structure for text.
#[derive(Debug, Clone, Default)]
pub struct Rope {
    pub text: String,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
    pub fn from_string(s: &str) -> Self {
        Self {
            text: s.to_string(),
        }
    }
    pub fn len(&self) -> usize {
        self.text.len()
    }
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
    pub fn char_at(&self, idx: usize) -> Option<char> {
        self.text.chars().nth(idx)
    }
    pub fn insert(&mut self, idx: usize, ch: char) {
        self.text.insert(idx, ch);
    }
    pub fn remove(&mut self, idx: usize) {
        self.text.remove(idx);
    }
    pub fn slice(&self, start: usize, end: usize) -> String {
        self.text
            .chars()
            .skip(start)
            .take(end.saturating_sub(start))
            .collect()
    }
    pub fn as_str(&self) -> &str {
        &self.text
    }
}
