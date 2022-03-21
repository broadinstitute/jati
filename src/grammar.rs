use crate::todet::ToDet;
use crate::token_result::TokenResult;

pub(crate) trait Grammar<I> {
    fn first_item(&self, item: &I) -> TokenResult<I>;
}

pub struct Map<I, A, R, F: Fn(A) -> R> {
    arg_todet: Box<dyn ToDet<I, A>>,
    result_todet: Box<dyn ToDet<I, R>>,
    map: F
}