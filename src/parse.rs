use crate::code_point::CodePoint;
use crate::error::ParseError;
use crate::line_break::LinuxOrWindowsLineBreaker;
use crate::parser::Parser;
use crate::token::code_point::CodePointIter;
use crate::token::token_iter::TokenIter;

pub fn parse_string<A, T: TokenIter<Item=CodePoint>>(string: &'static str,
                       parser: Box<dyn Parser<A, CodePoint, T>>)
                       -> Result<A, ParseError> {
    let line_breaker = LinuxOrWindowsLineBreaker::new();
    let byte_iter = string.bytes();
    let code_point_iter =
        CodePointIter::new(Box::new(byte_iter), Box::new(line_breaker));
    parser.parse(Box::new(code_point_iter))
}
