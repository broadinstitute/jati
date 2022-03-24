mod map;

use crate::error::ParseError;
use crate::token::Token;
use crate::token::token_iter::map::Map;

pub(crate) type TokenResult<I> = Result<Token<I>, ParseError>;
pub(crate) type TIterBox<I> = Box<dyn TIter<Item=I>>;

pub(crate) trait TIter {
    type Item;
}

pub trait TokenIter: TIter {
    fn map<J, F: FnMut(Self::Item) -> J>(self, f: F) -> Map<Self::Item, Self, J, F>
        where Self: Sized;
}

impl<I> TIter for TIterBox<I> {
    type Item = I;
}

impl<I> TokenIter for TIterBox<I> {
    fn map<J, F: FnMut(I) -> J>(self, f: F) -> Map<I, Self, J, F> {
        self.map(f)
    }
}

