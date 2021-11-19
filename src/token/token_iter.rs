use crate::error::Error;
use crate::token::Token;

pub(crate) type TokenResult<T> = Result<Token<T>, Error>;

pub(crate) trait TokenIter<T> : Iterator<Item=TokenResult<T>> {

}

