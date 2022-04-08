use crate::error::ParseError;
use crate::token::Token;
use crate::sub_queue::Key;
use crate::parser::Parser;

pub enum ParseMatch<I> {
    Ongoing(Box<dyn Parser<I>>),
    Complete(Complete<I>),
    Error(ParseError)
}

pub struct Complete<I> {
    deletion: Vec<Key>,
    insertion: Vec<Token<I>>
}

