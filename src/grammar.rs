use crate::todet::ToDet;
use crate::parse_run::ParseRun;

pub trait Grammar<I> {
    fn first_item(&self, item: &I) -> Option<Box<dyn ParseRun<I>>>;
}

pub struct Map<I, A, R, F: Fn(A) -> R> {
    arg_todet: Box<dyn ToDet<I, A>>,
    result_todet: Box<dyn ToDet<I, R>>,
    map: F
}