use crate::error::ParseError;
use crate::token::Token;

pub(crate) type TokenResult<T> = Result<Token<T>, ParseError>;

pub trait TokenIter<T: 'static> : Iterator<Item=TokenResult<T>> {

}

