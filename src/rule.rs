use crate::sub_queue::Key;
use crate::token::Token;

pub(crate) trait Rule<I> {
    fn apply(&self, key: Key, item: &I) -> Vec<MatchPath<I>>;
}

pub(crate) enum MatchPath<I> {
    Incomplete(Box<dyn Rule<I>>),
    Complete(Substitution<I>)
}

pub struct Substitution<I> {
    deletion: Vec<Key>,
    insertion: Vec<Token<I>>
}
