use crate::error::ParseError;
use crate::token::Token;

pub enum ParseMatch<I> {
    Ongoing(Box<dyn Ongoing<I>>),
    Complete(Complete<I>),
    Error(ParseError)
}

pub trait Ongoing<I> {
    fn apply_next(&self, token: Token<I>) -> ParseMatch<I>;
}

pub struct Complete<I> {
    item: I
}

