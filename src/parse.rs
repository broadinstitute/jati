use crate::char_pattern::CharPattern;
use crate::error::Error;
use crate::input::{CharTap, Input, Pos};
use crate::parse::parsers::alt::opt::OptParser;
use parsers::base::map::MapParser;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

pub(crate) mod error;
pub mod parsers;

pub struct Success<C: CharTap, O> {
    output: O,
    remainder: Input<C>,
}
pub trait Parser {
    type Output;
    fn parse<C: CharTap>(&self, input: &Input<C>) -> Result<Success<C, Self::Output>, ParseIssue>;
    fn opt(self) -> OptParser<Self::Output, Self>
    where
        Self: Sized,
    {
        OptParser::new(self)
    }
    fn map<O2, F>(self, f: F) -> MapParser<Self::Output, O2, Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O2,
    {
        MapParser::new(self, f)
    }
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

impl Failure {
    pub fn alt_combine(self, other: Failure) -> Failure {
        match self.pos.cmp(&other.pos) {
            Ordering::Less => other,
            Ordering::Equal => {
                let expected = self.expected.union(other.expected);
                Failure {
                    pos: self.pos,
                    actual: self.actual,
                    expected,
                }
            }
            Ordering::Greater => self,
        }
    }
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
            None => {
                write!(f, "got end of input, but expected ")?;
            }
            Some(c) => {
                write!(f, "got '{}', but expected ", c)?;
            }
        }
        write!(f, "{}", &self.expected)
    }
}
