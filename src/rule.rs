use crate::sub_queue::Key;
use crate::token::Token;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct Precedence {
    precedence: u8
}

impl From<u8> for Precedence {
    fn from(precedence: u8) -> Self { Precedence { precedence } }
}

pub(crate) enum Associativity {
    LeftToRight,
    RightToLeft
}

pub(crate) trait Rule<I> {
    fn precedence(&self) -> Precedence;
    fn associativity(&self) -> Associativity;
    fn apply(&self, key: Key, item: &I) -> Vec<Match<I>>;
}

pub struct Match<I> {
    path: Vec<Key>,
    application: Application<I>
}

pub(crate) enum Application<I> {
    Incomplete(Box<dyn Rule<I>>),
    Complete(Substitution<I>)
}

pub struct Substitution<I> {
    insertion: Vec<Token<I>>
}
