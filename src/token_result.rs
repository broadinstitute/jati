use crate::error::ParseError;
use crate::token::Token;

pub enum TokenResult<I> {
    Ongoing(Box<dyn Ongoing<I>>),
    Complete(Complete<I>),
    Error(ParseError)
}

pub trait Ongoing<I> {
    fn next_token(&self, token: Token<I>) -> TokenResult<I>;
}

pub struct Complete<I> {
    item: I
}

