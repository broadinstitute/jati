use crate::grammar::Grammar;
use crate::token::token_iter::TokenIterBox;

pub fn transform<A>(grammar: &dyn Grammar, token_iter: TokenIterBox<A>)
                    -> TokenIterBox<A> {
    todo!()
}

