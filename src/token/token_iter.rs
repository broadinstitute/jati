mod map;

use crate::error::ParseError;
use crate::token::Token;
use crate::token::token_iter::map::Map;

pub(crate) type TokenResult<I> = Result<Token<I>, ParseError>;

pub trait TokenIter<I>: Iterator<Item=TokenResult<I>> {
    fn map<J, F: FnMut(I) -> J>(self, f: F) -> Map<I, J, F>  where Self: Sized ;
}

impl<T, I> TokenIter<I> for T where T: Iterator<Item=TokenResult<I>> {
    fn map<J, F: FnMut(I) -> J>(self, f: F) -> Map<I, J, F>  where Self: Sized {
        todo!()
    }
}

// impl<I> TokenIter<I> for Box<dyn TokenIter<I>> {
//     fn map<J, F: FnMut(I) -> J>(self, f: F) -> Map<I, J, F> {
//         todo!()
//     }
// }

