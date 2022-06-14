use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Pos {
    byte_count: usize,
    char_count: usize,
    line_count: usize,
    char_in_line_count: usize,
}

impl Pos {
    fn line(&self) -> usize { self.line_count + 1 }
    fn col(&self) -> usize { self.char_in_line_count + 1 }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} (after {} chars, {} bytes)", self.line(), self.col(), self.char_count,
               self.byte_count)
    }
}