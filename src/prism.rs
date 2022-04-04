pub trait Prism<T, I> {
    fn merge(&self, item: I) -> T;
    fn check(&self, token: T) -> bool;
    fn inspect(&self, token: &T) -> Option<&I>;
    fn split(&self, token: T) -> Option<I>;
}
