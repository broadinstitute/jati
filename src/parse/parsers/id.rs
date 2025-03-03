use crate::error::Error;
use crate::input::Input;
use crate::parse::{ParseResult, Parser};

pub struct IdParser {}

impl IdParser {
    pub fn new() -> IdParser { IdParser {} }
}

impl Default for IdParser {
    fn default() -> Self { Self::new() }
}

impl Parser for IdParser {
    type Output = String;

    fn parse<C: Iterator<Item=Result<char, Error>> + Clone, I: Into<Input<C>>>(input: I)
        -> ParseResult<Self::Output> {
        unimplemented!()
    }
}