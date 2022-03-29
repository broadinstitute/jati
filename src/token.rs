use crate::pos::Pos;

pub mod code_point;
pub mod token_iter;

pub struct Token<I> {
    pub(crate) item: I,
    pos_begin: Pos,
    pos_end: Pos
}

impl<I> Token<I> {
    pub(crate) fn new(item: I, pos_begin: Pos, pos_end: Pos) -> Token<I> {
        Token { item, pos_begin, pos_end }
    }
}
