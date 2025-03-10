use crate::char_pattern::CharPattern;
use crate::input::{CharTap, Input};
use crate::parse::{Failure, ParseIssue, Parser};

pub struct CharParser {
    char_pattern: CharPattern
}

impl CharParser {
    pub fn new(char_pattern: CharPattern) -> Self {
        CharParser { char_pattern }
    }
}

impl Parser for CharParser {
    type Output = char;

    fn parse<C: CharTap>(&self, input: &mut Input<C>) -> Result<Self::Output, ParseIssue> {
        match input.next().transpose()? {
            None => {
                let pos = input.last_pos();
                Err(ParseIssue::Failure(Failure {
                    pos,
                    actual: None,
                    expected: Some(self.char_pattern.clone()),
                }))
            }
            Some(c) => {
                if self.char_pattern.includes(Some(c)) {
                    Ok(c)
                } else {
                    let pos = input.last_pos();
                    Err(ParseIssue::Failure(Failure {
                        pos,
                        actual: Some(c),
                        expected: Some(self.char_pattern.clone()),
                    }))
                }
            }
        }
    }
}

