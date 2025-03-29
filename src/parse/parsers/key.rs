use crate::char_pattern::CharPattern;
use crate::input::{CharTap, Input};
use crate::parse::{ParseIssue, Parser, Success};
use crate::parse::parsers::char::CharParser;

pub struct KeyParser {
    key: String,
}

impl KeyParser {
    pub fn new(key: String) -> Self {
        KeyParser { key }
    }
}

impl Parser for KeyParser {

    type Output = ();
    fn parse<C: CharTap>(&self, input: &Input<C>) -> Result<Success<C, ()>, ParseIssue> {
        let mut input = input.clone();
        for c in self.key.chars() {
            let char_parser = CharParser::new(CharPattern::for_char(c));
            let Success { input: next_input, output: _ } = char_parser.parse(&input)?;
            input = next_input;
        }
        Ok(Success { output: (), input })
    }
}