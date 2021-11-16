use crate::error::Error;
use crate::token::Token;

pub enum ItemRes<I, E> {
    Item(I),
    Err(E),
    End
}

pub trait ResIterOld<Item, Error> {
    fn next(&mut self) -> ItemRes<Item, Error>;
}

pub(crate) type TokenResult<T> = Result<Token<T>, Error>;

pub(crate) struct TokenIter<T> {
    iter: Box<dyn Iterator<Item=TokenResult<T>>>
}


