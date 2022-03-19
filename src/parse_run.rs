pub trait ParseRun<I> {
    fn next_item(&self, item: I) -> Option<Box<dyn ParseRun<I>>>;
}