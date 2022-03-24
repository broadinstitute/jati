use crate::token::token_iter::TokenIter;

pub struct Map<I, T: TokenIter<Item=I>, J, F: FnMut(I) -> J> {
    iter: T,
    f: F
}


