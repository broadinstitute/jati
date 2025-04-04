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
        Ok(Success { output: c, remainder: next.input })
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
        assert_parse(CharPattern::End, "", None);
        assert_parse(CharPattern::for_class(CharClass::Alphabetic), "a", Some('a'));
        assert_parse(CharPattern::for_class(CharClass::Alphanumeric), "a", Some('a'));
        assert_parse(CharPattern::for_class(CharClass::Alphanumeric), "1", Some('1'));
        assert_parse(CharPattern::for_char('='), "=", Some('='));
        let union =
            CharPattern::for_class(CharClass::Alphabetic).union(CharPattern::for_char('='));
        assert_parse(union.clone(), "=", Some('='));
        assert_parse(union, "=", Some('='));
    }

    fn assert_parse(char_pattern: CharPattern, string: &str, output: Option<char>) {
        let input = Input::from(string);
        let parser = CharParser::new(char_pattern);
        let result = parser.parse(&input);
        assert_eq!(result.unwrap().output, output);
    }

    #[test]
    fn test_char_parser_fail() {
        assert_failure(CharPattern::End, "1", Some('1'));
        assert_failure(CharPattern::for_class(CharClass::Alphabetic), "", None);
        assert_failure(CharPattern::for_class(CharClass::Alphabetic), "1", Some('1'));
        let union =
            CharPattern::for_class(CharClass::Alphabetic).union(CharPattern::for_char('='));
        assert_failure(union.clone(), "1", Some('1'));
        assert_failure(union, "+", Some('+'));
    }

    fn assert_failure(char_pattern: CharPattern, string: &str, actual: Option<char>) {
        let input = Input::from(string);
        let parser = CharParser::new(char_pattern);
        let result = parser.parse(&input);
        match result {
            Ok(_) => panic!("Expected failure."),
            Err(ParseIssue::Failure(failure)) => {
                assert_eq!(failure.actual, actual);
                assert_eq!(failure.expected.to_string(), parser.char_pattern.to_string());
            }
            Err(_) => panic!("Expected failure."),
        }
    }
}
