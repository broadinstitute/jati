use crate::error::ParseError;
use crate::token::Token;

pub(crate) type TokenResult<T> = Result<Token<T>, ParseError>;

pub(crate) type TokenIterBox<T> = Box<dyn Iterator<Item=TokenResult<T>>>;

