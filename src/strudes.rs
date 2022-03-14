trait Strudes<T, S, I> {
    fn tag() -> T;
    fn structure(item: I) -> S;
    fn check(structure: S) -> bool;
    fn inspect(structure: &S) -> Option<&I>;
    fn extract(structure: S) -> Option<I>;
}
