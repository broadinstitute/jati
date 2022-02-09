use crate::token::token_iter::TokenIterBox;
use crate::error::ParseError;
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

pub struct Jati {}

impl Jati {
    pub fn transform<A>(grammar: Box<dyn Grammar>, token_iter: TokenIterBox<A>)
        -> TokenIterBox<A> {
        todo!()
    }
    pub fn finish<A>(grammar: Box<dyn Grammar>, token_iter: TokenIterBox<A>)
        -> Result<A, ParseError> {
        todo!()
    }
}