use crate::token::token_iter::TokenIter;

pub struct Map<I, T: TokenIter<Item=I>, J, F: FnMut(I) -> J> {
    iter: T,
    f: F,
}

impl<I, T: TokenIter<Item=I>, J, F: FnMut(I) -> J> Map<I, T, J, F> {
    pub(crate) fn new(iter: T, f: F) -> Map<I, T, J, F> {
        Map { iter, f }
    }
}


