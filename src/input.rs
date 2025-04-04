use crate::char_pattern::CharPattern;
use crate::error::Error;
use crate::parse::Failure;
use std::fmt::Display;
use std::iter::Map;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone)]
pub struct PosChar {
    pub pos: Pos,
    pub char_opt: Option<char>,
}

#[derive(Clone)]
enum LineEndState { Lf1, Cr1, Lf2, Cr2, Other, }

#[derive(Clone)]
struct PosTracker {
    pos: Pos,
    line_end: LineEndState,
}

struct LineStateNext {
    state: LineEndState,
    new_line: bool,
}

impl LineEndState {
    fn new() -> Self {
        LineEndState::Other
    }
    fn next(&mut self, c: &Option<char>) -> LineStateNext {
        match c {
            Some(c) => match (self, c) {
                (LineEndState::Lf1, '\n') => LineStateNext {
                    state: LineEndState::Lf1,
                    new_line: true,
                },
                (LineEndState::Lf1, '\r') => LineStateNext {
                    state: LineEndState::Cr2,
                    new_line: false,
                },
                (LineEndState::Lf1, _) => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                (LineEndState::Cr1, '\n') => LineStateNext {
                    state: LineEndState::Lf2,
                    new_line: false,
                },
                (LineEndState::Cr1, '\r') => LineStateNext {
                    state: LineEndState::Cr1,
                    new_line: true,
                },
                (LineEndState::Cr1, _) => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                (LineEndState::Lf2, '\n') => LineStateNext {
                    state: LineEndState::Lf1,
                    new_line: true,
                },
                (LineEndState::Lf2, '\r') => LineStateNext {
                    state: LineEndState::Cr1,
                    new_line: true,
                },
                (LineEndState::Lf2, _) => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                (LineEndState::Cr2, '\n') => LineStateNext {
                    state: LineEndState::Lf1,
                    new_line: true,
                },
                (LineEndState::Cr2, '\r') => LineStateNext {
                    state: LineEndState::Cr1,
                    new_line: true,
                },
                (LineEndState::Cr2, _) => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                (LineEndState::Other, '\n') => LineStateNext {
                    state: LineEndState::Lf1,
                    new_line: false,
                },
                (LineEndState::Other, '\r') => LineStateNext {
                    state: LineEndState::Cr1,
                    new_line: false,
                },
                (LineEndState::Other, _) => LineStateNext {
                    state: LineEndState::Other,
                    new_line: false,
                },
            },
            None => match self {
                LineEndState::Lf1 => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                LineEndState::Cr1 => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                LineEndState::Lf2 => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                LineEndState::Cr2 => LineStateNext {
                    state: LineEndState::Other,
                    new_line: true,
                },
                LineEndState::Other => LineStateNext {
                    state: LineEndState::Other,
                    new_line: false,
                },
            },
        }
    }
}

pub trait CharTap: Iterator<Item = Result<char, Error>> + Clone {}

impl<T: Iterator<Item = Result<char, Error>> + Clone> CharTap for T {}

#[derive(Clone)]
pub struct Input<C: CharTap> {
    chars: C,
    pos_tracker: PosTracker,
}

pub struct Next<C: CharTap> {
    pub c: Option<char>,
    pub input: Input<C>,
}

impl Pos {
    pub fn new() -> Self {
        Pos { line: 1, column: 0 }
    }
    pub fn next(&self) -> Self {
        Pos {
            line: self.line,
            column: self.column + 1,
        }
    }
    pub fn newline(&self) -> Self {
        Pos {
            line: self.line + 1,
            column: 1,
        }
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
impl PosTracker {
    pub(crate) fn new() -> Self {
        PosTracker {
            pos: Pos::new(),
            line_end: LineEndState::new(),
        }
    }
    pub(crate) fn next(&mut self, c: &Option<char>) -> PosTracker {
        let LineStateNext { state, new_line } = self.line_end.next(c);
        let pos = if new_line {
            self.pos.newline()
        } else {
            self.pos.next()
        };
        PosTracker {
            pos,
            line_end: state,
        }
    }
}

impl<C: CharTap> Input<C> {
    pub fn new(chars: C) -> Self {
        Input {
            chars,
            pos_tracker: PosTracker::new(),
        }
    }
    pub fn last_pos(&self) -> Pos {
        self.pos_tracker.pos
    }
    pub fn the_next(&self) -> Result<Next<C>, Error> {
        let mut chars_next = self.chars.clone();
        let c = chars_next.next().transpose()?;
        let pos_tracker = self.pos_tracker.clone().next(&c);
        let input_next = Input { chars: chars_next, pos_tracker, };
        Ok(Next { c, input: input_next })
    }
}

impl<C: CharTap> Next<C> {
    pub fn match_with(&self, char_pattern: &CharPattern) -> Result<Option<char>, Failure> {
        if char_pattern.includes(self.c) {
            Ok(self.c)
        } else {
            let pos = self.input.last_pos();
            Err(Failure {
                pos,
                actual: self.c,
                expected: char_pattern.clone(),
            })
        }
    }
}

impl<I: Iterator<Item=char> + Clone> From<I> for Input<Map<I, fn(char) -> Result<char, Error>>> {
    fn from(chars: I) -> Self {
        Input::new(chars.map(Ok))
    }
}

impl<'a> From<&'a str> for Input<Map<std::str::Chars<'a>, fn(char) -> Result<char, Error>>>
{
    fn from(s: &'a str) -> Self {
        Input::new(s.chars().map(Ok))
    }
}