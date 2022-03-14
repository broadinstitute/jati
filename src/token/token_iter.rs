use crate::error::ParseError;
use crate::token::Token;

pub(crate) type TokenResult<T, I> = Result<Token<T, I>, ParseError>;

pub(crate) type TokenIterBox<T, I> = Box<dyn Iterator<Item=TokenResult<T, I>>>;

