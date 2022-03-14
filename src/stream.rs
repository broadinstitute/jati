use crate::error::ParseError;
use crate::token::Token;

trait Stream {
    type Tag;
    type Item;
    fn next(&mut self) -> Result<Token<Self::Tag, Self::Item>, ParseError>;
}