use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot<T> {
    pub state: T,
    pub timestamp: std::time::Instant,
}

impl<T> Snapshot<T> {
    pub fn new(state: T) -> Self {
        Self {
            state,
            timestamp: std::time::Instant::now(),
        }
    }
}

pub struct TimeTravel<T: Clone + PartialEq + Send + Sync + 'static> {
    history: VecDeque<T>,
    future: VecDeque<T>,
    limit: usize,
    current: T,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> TimeTravel<T> {
    pub fn new(initial: T, limit: usize) -> Self {
        Self {
            history: VecDeque::new(),
            future: VecDeque::new(),
            limit,
            current: initial,
        }
    }
    pub fn push(&mut self, state: T) {
        self.history.push_back(self.current.clone());
        if self.history.len() > self.limit {
            self.history.pop_front();
        }
        self.future.clear();
        self.current = state;
    }
    pub fn undo(&mut self) -> Option<T> {
        let previous = self.history.pop_back()?;
        self.future.push_back(self.current.clone());
        self.current = previous.clone();
        Some(previous)
    }
    pub fn redo(&mut self) -> Option<T> {
        let next = self.future.pop_back()?;
        self.history.push_back(self.current.clone());
        self.current = next.clone();
        Some(next)
    }
    pub fn current(&self) -> &T {
        &self.current
    }
    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }
    pub fn clear(&mut self) {
        self.history.clear();
        self.future.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_works() {
        let mut tt = TimeTravel::new(0_i32, 10);
        tt.push(1);
        tt.push(2);
        assert_eq!(tt.current(), &2);
        assert_eq!(tt.undo(), Some(1));
        assert_eq!(tt.current(), &1);
        assert_eq!(tt.redo(), Some(2));
        assert_eq!(tt.current(), &2);
        assert!(tt.can_undo());
        assert!(!tt.can_redo());
    }
}
