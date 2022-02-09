use crate::token::token_iter::TokenIterBox;
use crate::error::ParseError;

pub mod parser;
mod token;
mod pos;
mod code_point;
mod util;
mod error;
mod line_break;
pub mod parse;

pub struct Jati {}

impl Jati {
    pub fn transform<A, B>(token_iter: TokenIterBox<A>) -> TokenIterBox<B> {
        todo!()
    }
    pub fn finish<A, B>(token_iter: TokenIterBox<A>) -> Result<B, ParseError> {
        todo!()
    }
}