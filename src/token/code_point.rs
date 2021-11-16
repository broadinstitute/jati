use crate::pos::Pos;
use crate::code_point::CodePoint;
use crate::token::token_iter::TokenResult;
use crate::error::Error;
use crate::token::token_iter::TokenIter;

pub(crate) struct CodePointIter {
    bytes_iter: Box<dyn Iterator<Item=u8>>,
    pos: Pos,
    failure_opt: Option<Error>
}

impl CodePointIter {
    fn new(bytes_iter: Box<dyn Iterator<Item=u8>>) -> CodePointIter {
        let pos = Pos::new();
        let failure_opt = None;
        CodePointIter { bytes_iter, pos, failure_opt }
    }
}
