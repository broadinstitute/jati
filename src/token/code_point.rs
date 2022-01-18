use crate::pos::Pos;
use crate::code_point::CodePoint;
use crate::token::token_iter::{TokenResult, TokenIter};
use crate::error::Error;
use crate::line_break::LineBreaker;
use crate::token::Token;

pub(crate) struct CodePointIter {
    bytes_iter: Box<dyn Iterator<Item=u8>>,
    line_breaker: Box<dyn LineBreaker>,
    pos: Pos,
    failure_opt: Option<Error>,
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
        match CodePoint::read(&mut self.bytes_iter) {
            None => { None }
            Some(Ok(code_point)) => {
                let pos_raw = self.pos.add_char(code_point.n_utf8_bytes() as usize);
                let pos =
                    if self.line_breaker.check_if_new_line(&code_point) {
                        pos_raw.break_line()
                    } else {
                        pos_raw
                    };
                Some(Ok(Token::new(code_point, pos)))
            }
            Some(Err(utf8_error)) => {
                let pos = self.pos.add_char(utf8_error.i_byte as usize);
                Some(Err(Error::Utf8(utf8_error, pos)))
            }
        }
    }
}

impl TokenIter<CodePoint> for CodePointIter {}