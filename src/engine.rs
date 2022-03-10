use crate::grammar::Grammar;
use crate::token::token_iter::TokenIterBox;
use crate::sub_queue::SubQueue;
use crate::token::Token;
use crate::error::ParseError;

struct Engine<A> {
    token_iter: TokenIterBox<A>,
    grammar: Box<dyn Grammar>,
    queue: SubQueue<Token<A>>,
    got_none_from_iter: bool,
    error: Option<ParseError>,
}

impl<A> Engine<A> {
    pub(crate) fn new(token_iter: TokenIterBox<A>, grammar: Box<dyn Grammar>) -> Engine<A> {
        let queue = SubQueue::<Token<A>>::new();
        let got_none_from_iter = false;
        let error = None;
        Engine { token_iter, grammar, queue, got_none_from_iter, error }
    }
    fn add_token_to_queue(&mut self, token: Token<A>) {
        self.queue.push(token);
        todo!()
    }
    fn pull_next_token(&mut self) {
        match self.token_iter.next() {
            None => { self.got_none_from_iter = true; }
            Some(Err(error)) => { self.error = Some(error); }
            Some(Ok(token)) => { self.add_token_to_queue(token); }
        }
    }
    fn iterate(&mut self) {
        todo!()
    }
    pub(crate) fn next(&mut self) -> Option<Result<Token<A>, ParseError>> {
        todo!()
    }
    pub(crate) fn item(&mut self) -> Result<A, ParseError> {
        todo!()
    }
}
