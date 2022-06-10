use crate::token::token_iter::{TokenIter, TokenIterCore, TokenResult};

pub struct Lexing {
}

impl TokenIterCore for Lexing {
    type Item = ();

    fn next(&mut self) -> Option<TokenResult<Self::Item>> {
        todo!()
    }
}

impl TokenIter for Lexing {}
