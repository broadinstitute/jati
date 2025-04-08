use crate::char_pattern::{CharClass, CharPattern};
use crate::input::Input;
use crate::parse::parsers::base::char::CharParser;
use crate::parse::{ParseIssue, Parser, Success};

pub struct IdParser {}

impl IdParser {
    pub fn new() -> IdParser { IdParser {} }
}

impl Default for IdParser {
    fn default() -> Self { Self::new() }
}

impl Parser for IdParser {
    type Output = String;

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        let mut id = String::new();
        let start_char_parser =
            CharParser::new(CharPattern::for_class(CharClass::Alphabetic)
                .union(CharPattern::Char('_')));
        let Success { remainder: input, output: char1} = start_char_parser.parse(input)?;
        id.push(char1.unwrap());
        let char_parser =
            CharParser::new(CharPattern::for_class(CharClass::Alphanumeric)
                .union(CharPattern::Char('_')));
        let mut input = input;
        loop {
            match char_parser.parse(&input) {
                Ok(Success { remainder: next_input, output: c }) => {
                    id.push(c.unwrap());
                    input = next_input;
                }
                Err(ParseIssue::Failure(_)) => {
                    break Ok(Success { output: id, remainder: input });
                }
                Err(ParseIssue::Error(error)) => {
                    break Err(ParseIssue::Error(error));
                }
            }
        }
    }
}