use crate::pos::Pos;

pub(crate) mod code_point;
pub(crate) mod token_iter;

pub struct Token<T> {
    pub(crate) item: Box<T>,
    pos_begin: Pos,
    pos_end: Pos
}

impl<T: 'static> Token<T> {
    pub(crate) fn new(item: T, pos_begin: Pos, pos_end: Pos) -> Token<T> {
        let item = Box::new(item);
        Token { item, pos_begin, pos_end }
    }
}
