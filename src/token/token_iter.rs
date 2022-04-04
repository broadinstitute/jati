mod map;
mod lexing;

use crate::error::ParseError;
use crate::token::Token;
use crate::token::token_iter::map::Map;
use crate::token::token_iter::lexing::Lexing;
use crate::prism::Prism;
use crate::grammar::Grammar;

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
    fn lex<K, J, T>(self, in_prism: Box<dyn Prism<Self::Item, K>>, out_prism: Box<dyn Prism<J, K>>,
                    grammar: Box<dyn Grammar<K>>)
                    -> Lexing<Self::Item, K, J, Self>
        where Self: Sized {
        Lexing::new(self, in_prism, out_prism, grammar)
    }
}


