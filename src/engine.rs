use crate::grammar::Grammar;
use crate::token::token_iter::TokenIter;
use crate::sub_queue::SubQueue;
use crate::token::Token;
use crate::error::ParseError;

pub(crate) struct Engine<I, T: TokenIter<Item=I>> {
    token_iter: T,
    grammar: Box<dyn Grammar<I>>,
    queue: SubQueue<Token<I>>,
    got_none_from_iter: bool,
    error: Option<ParseError>,
}

impl<I, T: TokenIter<Item=I>> Engine<I, T> {
    pub(crate) fn new(token_iter: T, grammar: Box<dyn Grammar<I>>)
        -> Engine<I, T> {
        let queue = SubQueue::<Token<I>>::new();
        let got_none_from_iter = false;
        let error = None;
        Engine { token_iter, grammar, queue, got_none_from_iter, error }
    }
    fn add_token_to_queue(&mut self, token: Token<I>) {
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
    pub(crate) fn next(&mut self) -> Option<Result<Token<I>, ParseError>> {
        todo!()
    }
    pub(crate) fn item(&mut self) -> Result<I, ParseError> {
        todo!()
    }
}
