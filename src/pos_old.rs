use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct PosOld {
    count: usize,
    line: usize,
    col: usize,
}

impl PosOld {
    pub(crate) fn new() -> PosOld {
        PosOld { count: 0, line: 0, col: 0 }
    }
    pub(crate) fn next_in_line(&self) -> PosOld {
        PosOld { count: self.count + 1, line: self.line, col: self.col + 1 }
    }
    pub(crate) fn next_line(&self) -> PosOld {
        PosOld { count: self.count + 1, line: self.line + 1, col: 0 }
    }
}

impl Display for PosOld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.col, self.count)
    }
}