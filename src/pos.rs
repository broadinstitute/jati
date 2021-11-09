use std::fmt::{Display, Formatter};

pub struct Pos {
    byte_count: usize,
    char_count: usize,
    line_count: usize,
    char_in_line_count: usize,
}

impl Pos {
    fn new() -> Pos {
        Pos { byte_count: 0, char_count: 0, line_count: 0, char_in_line_count: 0 }
    }
    fn line(&self) -> usize { self.line_count + 1 }
    fn col(&self) -> usize { self.char_in_line_count + 1 }
    fn add_char(&self, n_bytes: usize) -> Pos {
        let byte_count = self.byte_count + n_bytes;
        let char_count = self.char_count + 1;
        let line_count = self.line_count;
        let char_in_line_count = self.char_in_line_count + 1;
        Pos { byte_count, char_count, line_count, char_in_line_count }
    }
    fn break_line(&self) -> Pos {
        let byte_count = self.byte_count;
        let char_count = self.char_count;
        let line_count = self.line_count + 1;
        let char_in_line_count = 0;
        Pos { byte_count, char_count, line_count, char_in_line_count }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} (after {} chars, {} bytes)", self.line(), self.col(), self.char_count,
               self.byte_count)
    }
}