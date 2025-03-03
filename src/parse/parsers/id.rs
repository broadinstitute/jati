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

    fn parse<B: Iterator<Item=u8> + Clone>(_bytes: B) -> ParseResult<Self::Output> {
        unimplemented!()
    }
}