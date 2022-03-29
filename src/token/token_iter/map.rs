use crate::token::token_iter::{TokenIter, TokenIterCore, TokenResult};
use crate::token::Token;

pub struct Map<I, T: TokenIter<Item=I>, J> {
    iter: T,
    f: Box<dyn FnMut(I) -> J>,
}

impl<I, T: TokenIter<Item=I>, J> Map<I, T, J> {
    pub(crate) fn new(iter: T, f: Box<dyn FnMut(I) -> J>) -> Map<I, T, J> {
        Map { iter, f }
    }
}

impl<I, T: TokenIter<Item=I>, J> TokenIterCore for Map<I, T, J> {
    type Item = J;

    fn next(&mut self) -> Option<TokenResult<Self::Item>> {
        self.iter.next().map(|token_result|{
            match token_result {
                Ok(token) => {
                    let item = (self.f)(token.item);
                    Ok(Token {
                        item,
                        pos_begin: token.pos_begin,
                        pos_end: token.pos_end
                    })
                }
                Err(error) => {
                    Err(error)
                }
            }
        })
    }
}

impl<I, T: TokenIter<Item=I>, J> TokenIter for Map<I, T, J> {

}


