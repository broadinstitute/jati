use crate::error::ParseError;
use crate::token::Token;

pub(crate) type TokenResult<I> = Result<Token<I>, ParseError>;

pub trait TokenIter<I>: Iterator<Item=TokenResult<I>> {

}

impl<T, I> TokenIter<I> for T where T: Iterator<Item=TokenResult<I>> {

}
