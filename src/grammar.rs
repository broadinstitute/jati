use crate::todet::Prism;
use crate::token_result::TokenResult;

pub(crate) trait Grammar<I> {
    fn first_item(&self, item: &I) -> TokenResult<I>;
}

pub struct Map<I, A, R, F: Fn(A) -> R> {
    arg_prism: Box<dyn Prism<I, A>>,
    result_prism: Box<dyn Prism<I, R>>,
    map: F
}