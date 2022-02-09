use crate::error::ParseError;
use crate::token::token_iter::TokenIterBox;

pub trait Parser<A, B: 'static> {
    fn parse(&self, token_iter: TokenIterBox<B>) -> Result<A, ParseError>;
}
