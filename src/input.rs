use std::mem;
use crate::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}


#[derive(Clone)]
enum CharClass {
    Lf, Cr, Other
}

#[derive(Clone)]
pub struct Input<C: Iterator<Item=Result<char, Error>> + Clone> {
    chars: C,
    last_pos: Option<Pos>,
    next_pos: Pos,
    last_char_class: CharClass,
}

impl Pos {
    pub fn new() -> Self {
        Pos { line: 1, column: 1 }
    }
    pub fn next(&self) -> Self {
        Pos { line: self.line, column: self.column + 1 }
    }
    pub fn newline(&self) -> Self {
        Pos { line: self.line + 1, column: 1 }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos::new()
    }
}

impl<C: Iterator<Item=Result<char, Error>> + Clone> Input<C> {
    pub fn new(chars: C) -> Self {
        Input { chars, last_pos: None, next_pos: Pos::new(), last_char_class: CharClass::Other }
    }
    pub fn last_pos(&self) -> Option<Pos> {
        self.last_pos
    }
}

impl<C: Iterator<Item=Result<char, Error>> + Clone> Iterator for Input<C> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            None => None,
            Some(Err(err)) => Some(Err(err)),
            Some(Ok(c)) => {
                let pos = self.next_pos;
                self.next_pos = match (c, &self.last_char_class) {
                    ('\n', CharClass::Cr) => { self.last_char_class = CharClass::Lf; pos }
                    ('\n', _) => { self.last_char_class = CharClass::Lf; pos.newline() }
                    ('\r', CharClass::Lf) => { self.last_char_class = CharClass::Cr; pos }
                    ('\r', _) => { self.last_char_class = CharClass::Cr; pos.newline() }
                    (_, _) => { self.last_char_class = CharClass::Other; pos.next() }
                };
                self.last_pos = Some(mem::replace(&mut self.next_pos, pos));
                Some(Ok(c))
            }
        }
    }
}