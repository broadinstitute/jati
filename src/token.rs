use crate::pos::Pos;

mod code_point;
pub(crate) mod token_iter;

pub struct Token<T> {
    item: T,
    pos: Pos,
}

impl<T> Token<T> {
    pub(crate) fn new(item: T, pos: Pos) -> Token<T> {
        Token { item, pos }
    }
}
