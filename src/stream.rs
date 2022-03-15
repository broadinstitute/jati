use crate::error::ParseError;
use crate::token::Token;

trait Stream {
    type Item;
    fn next(&mut self) -> Result<Token<Self::Item>, ParseError>;
}