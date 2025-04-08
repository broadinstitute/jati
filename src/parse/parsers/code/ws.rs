use crate::char_pattern::{CharClass, CharPattern};
use crate::input::Input;
use crate::parse::parsers::base::char::CharParser;
use crate::parse::{ParseIssue, Parser, Success};

pub struct WhitespaceParser {}

impl WhitespaceParser {
    pub fn new() -> Self { Self {} }
}

impl Default for WhitespaceParser {
    fn default() -> Self { Self::new() }
}

impl Parser for WhitespaceParser {
    type Output = ();

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        let mut input = input.clone();
        let ws_char_parser =
            CharParser::new(CharPattern::for_class(CharClass::Whitespace));
        loop {
            let input_previous = input.clone();
            match ws_char_parser.parse(&input) {
                Ok(Success { output: _, remainder }) => {
                    input = remainder;
                }
                Err(ParseIssue::Error(error)) => break Err(ParseIssue::Error(error)),
                Err(ParseIssue::Failure(_)) => {
                    break Ok(Success {
                        output: (),
                        remainder: input_previous,
                    });
                }
            }
        }
    }
}