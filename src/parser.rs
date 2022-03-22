use crate::error::ParseError;
use crate::token::token_iter::TokenIter;

pub trait Parser<A, I: 'static> {
    fn parse(&self, token_iter: Box<dyn TokenIter<I>>) -> Result<A, ParseError>;
}
