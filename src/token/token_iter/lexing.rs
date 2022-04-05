use crate::prism::Prism;
use crate::token::token_iter::{TokenIter, TokenIterCore, TokenResult};
use crate::grammar::Grammar;

pub struct Lexing<I, K, J, T: TokenIter<Item=I>> {
    iter: T,
    input_prism: Box<dyn Prism<I, K>>,
    output_prism: Box<dyn Prism<J, K>>,
    grammar: Box<dyn Grammar<K>>,
}

impl<I, K, J, T: TokenIter<Item=I>> Lexing<I, K, J, T> {
    pub(crate) fn new(iter: T, input_prism: Box<dyn Prism<I, K>>,
                      output_prism: Box<dyn Prism<J, K>>, grammar: Box<dyn Grammar<K>>)
                      -> Lexing<I, K, J, T> {
        Lexing { iter, input_prism, output_prism, grammar }
    }
}

impl<I, K, J, T: TokenIter<Item=I>> TokenIterCore for Lexing<I, K, J, T> {
    type Item = J;

    fn next(&mut self) -> Option<TokenResult<Self::Item>> {
        todo!()
    }
}

impl<I, K, J, T: TokenIter<Item=I>> TokenIter for Lexing<I, K, J, T> {}
