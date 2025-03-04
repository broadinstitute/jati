use crate::char_pattern::{CharClass, CharPattern};
use crate::input::{CharTap, Input};
use crate::parse::{ParseIssue, Parser};
use crate::parse::parsers::char::CharParser;

pub struct IdParser {}

impl IdParser {
    pub fn new() -> IdParser { IdParser {} }
}

impl Default for IdParser {
    fn default() -> Self { Self::new() }
}

impl Parser for IdParser {
    type Output = String;

    fn parse<C: CharTap>(&self, input: &mut Input<C>) -> Result<Self::Output, ParseIssue> {
        let mut id = String::new();
        let start_char_parser =
            CharParser::new(CharPattern::for_class(CharClass::Alphabetic).union(CharPattern::Char('_')));
        let char1 = start_char_parser.parse(input)?;
        id.push(char1);
        loop {
            let char_parser =
                CharParser::new(CharPattern::for_class(CharClass::Alphanumeric).union(CharPattern::Char('_')));
            match char_parser.parse(input) {
                Ok(c) => id.push(c),
                Err(ParseIssue::Failure(_)) => break Ok(id),
                Err(ParseIssue::Error(error)) => Err(ParseIssue::Error(error))?,
            }
        }
    }
}