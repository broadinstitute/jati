use crate::pos::Pos;

pub(crate) mod code_point;
pub(crate) mod token_iter;

pub struct Token<T, I> {
    tag: T,
    pub(crate) item: I,
    pos_begin: Pos,
    pos_end: Pos
}

impl<T, I> Token<T, I> {
    pub(crate) fn new(tag: T, item: I, pos_begin: Pos, pos_end: Pos) -> Token<T, I> {
        Token { tag, item, pos_begin, pos_end }
    }
}
