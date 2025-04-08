use crate::input::Input;
use crate::parse::{ParseIssue, Parser, Success};
use std::sync::Arc;

#[derive(Clone)]
pub struct BoxedParser<O, P: Parser<Output=O>> {
    pub inner: Arc<P>,
}

impl<O, P: Parser<Output=O>> BoxedParser<O, P> {
    pub fn new(inner: P) -> Self { BoxedParser { inner: Arc::new(inner) } }
}

impl<O, P: Parser<Output=O>> Parser for BoxedParser<O, P> {
    type Output = O;

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        self.inner.parse(input)
    }
}