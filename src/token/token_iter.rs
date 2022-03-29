mod map;

use crate::error::ParseError;
use crate::token::Token;
use crate::token::token_iter::map::Map;

pub(crate) type TokenResult<I> = Result<Token<I>, ParseError>;
pub(crate) type TokenIterCoreBox<I> = Box<dyn TokenIterCore<Item=I>>;

pub trait TokenIterCore {
    type Item;
    fn next(&mut self) -> Option<TokenResult<Self::Item>>;
}

pub trait TokenIter: TokenIterCore {
    fn map<J, F: 'static + FnMut(Self::Item) -> J>(self, f: F) -> Map<Self::Item, Self, J>
        where Self: Sized {
        Map::new(self, Box::new(f))
    }
}


