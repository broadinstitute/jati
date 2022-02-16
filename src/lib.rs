use crate::token::token_iter::TokenIterBox;
use crate::grammar::Grammar;

pub mod parser;
mod token;
mod pos;
mod code_point;
mod util;
mod error;
mod line_break;
pub mod parse;
mod grammar;
mod engine;
mod production;
mod stream;

pub struct Jati {}

impl Jati {
    pub fn transform<A>(&self, grammar: &dyn Grammar, token_iter: TokenIterBox<A>)
                        -> TokenIterBox<A> {
        engine::transform::<A>(grammar, token_iter)
    }
}