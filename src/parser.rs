use crate::error::ParseError;
use crate::token::token_iter::TokenIter;

pub trait Parser<A, B: 'static> {
    fn parse(&self, token_iter: Box<dyn TokenIter<B>>) -> Result<A, ParseError>;
}
