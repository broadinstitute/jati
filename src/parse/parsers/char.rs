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

