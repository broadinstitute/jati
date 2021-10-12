use std::fmt::{Display, Formatter};

pub(crate) struct Pos {
    pos: usize,
    line: usize,
    col: usize
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.col, self.pos)
    }
}