use crate::char_pattern::CharPattern;
use crate::error::Error;
use crate::input::{CharTap, Input, Pos};
use std::fmt::{Display, Formatter};

pub(crate) mod error;
pub mod parsers;

pub struct Success<C: CharTap, O> {
    output: O,
    input: Input<C>
}
pub trait Parser {
    type Output;
    fn parse<C: CharTap>(&self, input: &Input<C>)
        -> Result<Success<C, Self::Output>, ParseIssue>;
}
#[derive(Debug)]
pub enum ParseIssue {
    Error(Error),
    Failure(Failure),
}
#[derive(Debug)]
pub struct Failure {
    pub pos: Pos,
    pub actual: Option<char>,
    pub expected: CharPattern,
}

impl From<Error> for ParseIssue {
    fn from(e: Error) -> Self {
        ParseIssue::Error(e)
    }
}

impl From<Failure> for ParseIssue {
    fn from(f: Failure) -> Self {
        ParseIssue::Failure(f)
    }
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "At {}, ", self.pos)?;
        match self.actual {
            None => { write!(f, "got end of input, but expected ")?; }
            Some(c) => { write!(f, "got '{}', but expected ", c)?; }
        }
        write!(f, "{}", &self.expected)
    }
}