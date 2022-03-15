use crate::error::ParseError;
use crate::token::token_iter::TokenIterBox;

pub trait Parser<A, I: 'static> {
    fn parse(&self, token_iter: TokenIterBox<I>) -> Result<A, ParseError>;
}
