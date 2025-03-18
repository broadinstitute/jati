use crate::char_pattern::CharPattern;
use crate::input::{CharTap, Input};
use crate::parse::{ParseIssue, Parser, Success};

pub struct CharParser {
    char_pattern: CharPattern
}

impl CharParser {
    pub fn new(char_pattern: CharPattern) -> Self {
        CharParser { char_pattern }
    }
}

impl Parser for CharParser {
    type Output = Option<char>;

    fn parse<C: CharTap>(&self, input: &Input<C>) -> Result<Success<C, Self::Output>, ParseIssue> {
        let next = input.the_next()?;
        let c = next.match_with(&self.char_pattern)?;
        Ok(Success { output: c, input: next.input })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char_pattern::{CharClass, CharPattern};
    use crate::input::Input;
    use crate::parse::ParseIssue;

    #[test]
    fn test_char_parser() {
        let input = Input::from("a");
        let parser = CharParser::new(CharPattern::for_class(CharClass::Alphabetic));
        let result = parser.parse(&input);
        assert_eq!(result.unwrap().output, Some('a'));
    }

    #[test]
    fn test_char_parser_fail() {
        let input = Input::from("1");
        let parser = CharParser::new(CharPattern::for_class(CharClass::Alphabetic));
        let result = parser.parse(&input);
        match result {
            Ok(_) => panic!("Expected failure."),
            Err(ParseIssue::Failure(failure)) => {
                assert_eq!(failure.actual, Some('1'));
                assert_eq!(failure.expected.to_string(), CharClass::Alphabetic.to_string());
            }
            Err(_) => panic!("Expected failure."),
        }
    }
}
