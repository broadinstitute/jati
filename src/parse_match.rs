use crate::error::ParseError;
use crate::token::Token;
use crate::sub_queue::Key;

pub enum ParseMatch<I> {
    Ongoing(Box<dyn Ongoing<I>>),
    Complete(Complete<I>),
    Error(ParseError)
}

pub trait Ongoing<I> {
    fn apply_next(&self, key: Key, token: Token<I>) -> ParseMatch<I>;
    fn apply_end(&self) -> ParseMatch<I>;
}

pub struct Complete<I> {
    deletion: Vec<Key>,
    insertion: Vec<Token<I>>
}

