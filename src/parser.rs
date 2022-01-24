use crate::error::ParseError;
use crate::token::Token;

pub trait Parser<A, B> {
    fn parse(token_iter: Box<dyn Iterator<Item=Token<B>>>) -> Result<A, ParseError>;
}
