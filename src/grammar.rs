use crate::prism::Prism;
use crate::parse_match::ParseMatch;

pub trait Grammar<I> {
    fn apply_start(&self, item: &I) -> ParseMatch<I>;
}

pub struct Map<I, A, R, F: Fn(A) -> R> {
    arg_prism: Box<dyn Prism<I, A>>,
    result_prism: Box<dyn Prism<I, R>>,
    map: F
}