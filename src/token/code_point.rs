use crate::pos::Pos;
use crate::code_point::CodePoint;
use crate::token::token_iter::TokenResult;
use crate::error::ParseError;
use crate::line_break::LineBreaker;
use crate::token::Token;
use core::mem;

pub(crate) struct CodePointIter {
    bytes_iter: Box<dyn Iterator<Item=u8>>,
    line_breaker: Box<dyn LineBreaker>,
    pos_begin: Pos,
    pos_end: Pos,
    pos: Pos,
    failure_opt: Option<ParseError>,
}

impl CodePointIter {
    pub(crate) fn new(bytes_iter: Box<dyn Iterator<Item=u8>>, line_breaker: Box<dyn LineBreaker>)
                      -> CodePointIter {
        let pos_begin = Pos::new();
        let pos_end = Pos::new();
        let pos = Pos::new();
        let failure_opt = None;
        CodePointIter { bytes_iter, line_breaker, pos_begin, pos_end, pos, failure_opt }
    }
}

const TAG: &str = "code_point";

impl Iterator for CodePointIter {
    type Item = TokenResult<&'static str, CodePoint>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(error) = &self.failure_opt {
            Some(Err(error.clone()))
        } else {
            match CodePoint::read(&mut self.bytes_iter) {
                None => { None }
                Some(Ok(code_point)) => {
                    let pos_unbroken = self.pos.add_char(code_point.n_utf8_bytes() as usize);
                    let pos_end_new =
                        if self.line_breaker.check_if_new_line(&code_point) {
                            pos_unbroken.break_line()
                        } else {
                            pos_unbroken
                        };
                    self.pos_begin = mem::replace(&mut self.pos_end, pos_end_new);
                    Some(Ok(Token::new(TAG,code_point, self.pos_begin.clone(),
                                       self.pos_end.clone())))
                }
                Some(Err(utf8_error)) => {
                    let pos = self.pos.add_char(utf8_error.i_byte as usize);
                    let error = ParseError::Utf8(utf8_error, pos);
                    self.failure_opt = Some(error.clone());
                    Some(Err(error))
                }
            }
        }
    }
}
