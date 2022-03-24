use crate::error::ParseError;
use crate::token::token_iter::TokenIter;

pub trait Parser<A, I: 'static, T: TokenIter<Item=I>> {
    fn parse(&self, token_iter: T) -> Result<A, ParseError>;
}
