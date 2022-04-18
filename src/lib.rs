use crate::token::token_iter::TokenIter;
use crate::code_point::CodePoint;
use crate::line_break::LinuxOrWindowsLineBreaker;
use crate::token::code_point::CodePointIter;

pub mod token;
mod pos;
pub mod code_point;
mod util;
mod error;
mod line_break;
mod parser;
mod engine;
mod stream;
mod sub_queue;
mod prism;
mod rule;
mod rule_set;

pub fn scan_string(string: String) -> impl TokenIter<Item=CodePoint> {
    let bytes_iter = string.into_bytes().into_iter();
    let line_breaker = LinuxOrWindowsLineBreaker::new();
    CodePointIter::new(Box::new(bytes_iter), Box::new(line_breaker))
}
