use crate::prism::Prism;
use crate::parse_match::ParseMatch;
use crate::sub_queue::Key;

pub trait Grammar<I> {
    fn apply_start(&self, key: Key, item: &I) -> ParseMatch<I>;
}

pub struct Map<I, A, R, F: Fn(A) -> R> {
    arg_prism: Box<dyn Prism<I, A>>,
    result_prism: Box<dyn Prism<I, R>>,
    map: F
}