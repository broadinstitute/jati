use crate::token::token_iter::TokenIter;
use crate::code_point::CodePoint;
use crate::line_break::LinuxOrWindowsLineBreaker;
use crate::token::code_point::CodePointIter;

pub mod parser;
mod token;
mod pos;
mod code_point;
mod util;
mod error;
mod line_break;
pub mod parse;
mod grammar;
mod engine;
mod stream;
mod sub_queue;
mod todet;
mod parse_run;
mod token_result;

pub struct Jati {}

impl Jati {
    fn scan_string(string: String) -> Box<dyn TokenIter<CodePoint>> {
        let bytes_iter = string.into_bytes().into_iter();
        let line_breaker = LinuxOrWindowsLineBreaker::new();
        let code_point_iter =
            CodePointIter::new(Box::new(bytes_iter), Box::new(line_breaker));
        Box::new(code_point_iter)
    }
}