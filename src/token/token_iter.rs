use crate::error::ParseError;
use crate::token::Token;

pub(crate) type TokenResult<I> = Result<Token<I>, ParseError>;

pub(crate) type TokenIterBox<I> = Box<dyn Iterator<Item=TokenResult<I>>>;

