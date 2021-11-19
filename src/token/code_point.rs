use crate::pos::Pos;
use crate::code_point::CodePoint;
use crate::token::token_iter::{TokenResult, TokenIter};
use crate::error::Error;
use crate::line_break::LineBreaker;

pub(crate) struct CodePointIter {
    bytes_iter: Box<dyn Iterator<Item=u8>>,
    line_breaker: Box<dyn LineBreaker>,
    pos: Pos,
    failure_opt: Option<Error>
}

impl CodePointIter {
    fn new(bytes_iter: Box<dyn Iterator<Item=u8>>, line_breaker: Box<dyn LineBreaker>)
        -> CodePointIter {
        let pos = Pos::new();
        let failure_opt = None;
        CodePointIter { bytes_iter, line_breaker, pos, failure_opt }
    }
}

impl Iterator for CodePointIter {
    type Item = TokenResult<CodePoint>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos_new = &self.pos;

        todo!()
    }
}

impl TokenIter<CodePoint> for CodePointIter {

}