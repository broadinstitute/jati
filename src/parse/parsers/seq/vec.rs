use crate::input::{CharTap, Input};
use crate::parse::{ParseIssue, Parser, Success};

pub struct VecParser<T, P>
where
    P: Parser<Output = T>,
{
    pub parser: P,
    pub min: usize,
    pub max: Option<usize>,
}

impl<T, P> VecParser<T, P>
where
    P: Parser<Output = T>,
{
    pub fn new(parser: P, min: usize, max: Option<usize>) -> Self {
        VecParser { parser, min, max }
    }
}

impl<T, P> Parser for VecParser<T, P>
where
    P: Parser<Output = T>,
{
    type Output = Vec<T>;

    fn parse<C: CharTap>(&self, input: &Input<C>) -> Result<Success<C, Self::Output>, ParseIssue> {
        let mut result = Vec::new();
        let mut current_input = input.clone();
        loop {
            if let Some(max) = self.max {
                if result.len() >= max {
                    break Ok(Success {
                        output: result,
                        remainder: current_input,
                    });
                }
            }
            match self.parser.parse(&current_input) {
                Ok(Success { output, remainder }) => {
                    result.push(output);
                    current_input = remainder;
                }
                Err(ParseIssue::Error(error)) => break Err(ParseIssue::Error(error)),
                Err(ParseIssue::Failure(failure)) => {
                    if result.len() < self.min {
                        break Err(ParseIssue::Failure(failure));
                    } else {
                        break Ok(Success {
                            output: result,
                            remainder: current_input,
                        });
                    }
                }
            }
        }
    }
}
