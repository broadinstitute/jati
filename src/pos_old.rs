use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct PosOld {
    count: usize,
    line: usize,
    col: usize,
}

impl Display for PosOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.col, self.count)
    }
}