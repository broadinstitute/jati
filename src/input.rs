use std::fmt::Display;
use std::mem;
use crate::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone)]
pub struct PosChar {
    pub pos: Pos,
    pub c: Option<char>,
}

#[derive(Clone)]
enum LineState {
    Lf1, Cr1, Lf2, Cr2, Other
}

struct LineStateNext {
    state: LineState,
    new_line: bool,
}

impl LineState {
    fn new() -> Self { LineState::Other }
    fn next(&mut self, c: Option<char>) -> LineStateNext {
        match c {
            Some(c) => {
                match (self, c) {
                    (LineState::Lf1, '\n') => LineStateNext { state: LineState::Lf1, new_line: true },
                    (LineState::Lf1, '\r') => LineStateNext { state: LineState::Cr2, new_line: false },
                    (LineState::Lf1, _) => LineStateNext { state: LineState::Other, new_line: true },
                    (LineState::Cr1, '\n') => LineStateNext { state: LineState::Cr2, new_line: false },
                    (LineState::Cr1, '\r') => LineStateNext { state: LineState::Cr1, new_line: true },
                    (LineState::Cr1, _) => LineStateNext { state: LineState::Other, new_line: true },
                    (LineState::Lf2, '\n') => LineStateNext { state: LineState::Lf1, new_line: true },
                    (LineState::Lf2, '\r') => LineStateNext { state: LineState::Cr1, new_line: true },
                    (LineState::Lf2, _) => LineStateNext { state: LineState::Other, new_line: true },
                    (LineState::Cr2, '\n') => LineStateNext { state: LineState::Lf1, new_line: true },
                    (LineState::Cr2, '\r') => LineStateNext { state: LineState::Cr1, new_line: true },
                    (LineState::Cr2, _) => LineStateNext { state: LineState::Other, new_line: true },
                    (LineState::Other, '\n') => LineStateNext { state: LineState::Lf1, new_line: false },
                    (LineState::Other, '\r') => LineStateNext { state: LineState::Cr1, new_line: false },
                    (LineState::Other, _) => LineStateNext { state: LineState::Other, new_line: false },
                }
            }
            None => {
                match self {
                    LineState::Lf1 => LineStateNext { state: LineState::Other, new_line: true },
                    LineState::Cr1 => LineStateNext { state: LineState::Other, new_line: true },
                    LineState::Lf2 => LineStateNext { state: LineState::Other, new_line: true },
                    LineState::Cr2 => LineStateNext { state: LineState::Other, new_line: true },
                    LineState::Other => LineStateNext { state: LineState::Other, new_line: false },
                }
            }
        }
    }
}

pub trait CharTap: Iterator<Item=Result<char, Error>> + Clone {}

impl<T: Iterator<Item=Result<char, Error>> + Clone> CharTap for T {}

#[derive(Clone)]
pub struct Input<C: CharTap> {
    chars: C,
    next_char_pos: Result<PosChar, Error>,
    last_char_class: LineState,
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

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl<C: CharTap> Input<C> {
    pub fn new(mut chars: C) -> Self {
        let next_char_pos =
            chars.next().transpose().map(|c| PosChar { pos: Pos::new(), c });
        Input { chars, next_char_pos, last_char_class: LineState::Other }
    }
}

impl<C: CharTap> Iterator for Input<C> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            None => None,
            Some(Err(err)) => Some(Err(err)),
            Some(Ok(c)) => {
                let pos = self.next_pos;
                self.next_pos = match (c, &self.last_char_class) {
                    ('\n', LineState::Cr) => { self.last_char_class = LineState::Lf; pos }
                    ('\n', _) => { self.last_char_class = LineState::Lf; pos.newline() }
                    ('\r', LineState::Lf) => { self.last_char_class = LineState::Cr; pos }
                    ('\r', _) => { self.last_char_class = LineState::Cr; pos.newline() }
                    (_, _) => { self.last_char_class = LineState::Other; pos.next() }
                };
                self.last_pos = Some(mem::replace(&mut self.next_pos, pos));
                Some(Ok(c))
            }
        }
    }
}