use crate::pos::Pos;

mod code_point;
pub(crate) mod token_iter;

pub(crate) struct Token<T> {
    pos: Pos,
    item: T
}
