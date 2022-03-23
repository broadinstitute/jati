use crate::token::token_iter::TokenIter;

pub(crate) struct Map<I, J, F: FnMut(I) -> J> {
    iter: Box<dyn TokenIter<I>>,
    f: F
}


