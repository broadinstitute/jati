use crate::prism::Prism;
use crate::token_result::TokenResult;

pub trait Grammar<I> {
    fn first_item(&self, item: &I) -> TokenResult<I>;
}

pub struct Map<I, A, R, F: Fn(A) -> R> {
    arg_prism: Box<dyn Prism<I, A>>,
    result_prism: Box<dyn Prism<I, R>>,
    map: F
}