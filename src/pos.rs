use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Pos {
    count: usize,
    line: usize,
    col: usize,
}

impl Pos {
    pub(crate) fn new() -> Pos {
        Pos { count: 0, line: 0, col: 0 }
    }
    pub(crate) fn next_in_line(&self) -> Pos {
        Pos { count: self.count + 1, line: self.line, col: self.col + 1 }
    }
    pub(crate) fn next_line(&self) -> Pos {
        Pos { count: self.count + 1, line: self.line + 1, col: 0 }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.col, self.count)
    }
}