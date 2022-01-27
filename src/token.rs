use crate::pos::Pos;
use std::any::Any;

pub(crate) mod code_point;
pub(crate) mod token_iter;

pub struct Token<T: Any> {
    item: Box<T>,
    pos: Pos,
}

impl<T: 'static> Token<T> {
    pub(crate) fn new(item: T, pos: Pos) -> Token<T> {
        let item = Box::new(item);
        Token { item, pos }
    }
}
