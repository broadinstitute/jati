use crate::error::ParseError;
use crate::token::token_iter::TokenIterBox;

pub trait Parser<A, T, I: 'static> {
    fn parse(&self, token_iter: TokenIterBox<T, I>) -> Result<A, ParseError>;
}
