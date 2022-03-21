use crate::error::ParseError;
use crate::token::Token;

pub(crate) enum TokenResult<I> {
    Ongoing(Box<dyn Ongoing<I>>),
    Complete(Complete<I>),
    Error(ParseError)
}

pub(crate) trait Ongoing<I> {
    fn next_token(&self, token: Token<I>) -> TokenResult<I>;
}

pub(crate) struct Complete<I> {
    item: I
}

