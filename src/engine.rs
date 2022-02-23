use crate::grammar::Grammar;
use crate::token::token_iter::TokenIterBox;
use crate::sub_queue::SubQueue;
use crate::token::Token;
use crate::error::ParseError;

struct Engine<A> {
    token_iter: TokenIterBox<A>,
    grammar: Box<dyn Grammar>,
    queue: SubQueue<A>,
}

impl<A> Engine<A> {
    pub(crate) fn new(token_iter: TokenIterBox<A>, grammar: Box<dyn Grammar>) -> Engine<A> {
        let queue = SubQueue::<A>::new();
        Engine { token_iter, grammar, queue }
    }
    pub(crate) fn next(&mut self) -> Option<Result<Token<A>, ParseError>> {
        todo!()
    }
    pub(crate) fn item(&mut self) -> Result<A, ParseError> {
        todo!()
    }
}
