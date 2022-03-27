use crate::token::token_iter::TokenIter;
use crate::code_point::CodePoint;
use crate::line_break::LinuxOrWindowsLineBreaker;
use crate::token::code_point::CodePointIter;

mod token;
mod pos;
mod code_point;
mod util;
mod error;
mod line_break;
mod grammar;
mod engine;
mod stream;
mod sub_queue;
mod todet;
mod parse_run;
mod token_result;

pub struct Jati {}

impl Jati {
    pub fn scan_string(string: String) -> impl TokenIter<Item=CodePoint> {
        let bytes_iter = string.into_bytes().into_iter();
        let line_breaker = LinuxOrWindowsLineBreaker::new();
        CodePointIter::new(Box::new(bytes_iter), Box::new(line_breaker))
    }
}