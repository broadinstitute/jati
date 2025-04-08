use crate::char_pattern::CharPattern;
use crate::input::Input;
use crate::parse::parsers::base::char::CharParser;
use crate::parse::{ParseIssue, Parser, Success};

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
    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, ()>, ParseIssue> {
        let mut input = input.clone();
        for c in self.key.chars() {
            let char_parser = CharParser::new(CharPattern::for_char(c));
            let Success { remainder: next_input, output: _ } = char_parser.parse(&input)?;
            input = next_input;
        }
        Ok(Success { output: (), remainder: input })
    }
}