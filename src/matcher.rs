use crate::error::ParseError;
use crate::token::Token;
use crate::sub_queue::Key;
use std::marker::PhantomData;

pub(crate) struct Matcher<I> {
    ghost: PhantomData<I>
}

impl<I> Matcher<I> {
    fn apply_start(&self, key: Key, item: &I) -> MatcherResult<I> {
        todo!()
    }
    fn apply_end(&self) -> MatcherResult<I> {
        todo!()
    }
}

pub enum MatcherResult<I> {
    Ongoing(Matcher<I>),
    Complete(Substitution<I>),
    Error(ParseError)
}


pub struct Substitution<I> {
    deletion: Vec<Key>,
    insertion: Vec<Token<I>>
}

