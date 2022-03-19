pub trait ToDet<T, I> {
    fn tokenize(&self, item: I) -> T;
    fn check(&self, token: T) -> bool;
    fn inspect(&self, token: &T) -> Option<&I>;
    fn detokenize(&self, token: T) -> Option<I>;
}
