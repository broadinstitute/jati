use std::sync::Arc;
use crate::input::{CharTap, Input};
use crate::parse::{ParseIssue, Parser, Success};

#[derive(Clone)]
pub struct BoxedParser<O, P: Parser<Output=O>> {
    pub inner: Arc<P>,
}

impl<O, P: Parser<Output=O>> BoxedParser<O, P> {
    pub fn new(inner: P) -> Self { BoxedParser { inner: Arc::new(inner) } }
}

impl<O, P: Parser<Output=O>> Parser for BoxedParser<O, P> {
    type Output = O;

    fn parse<C: CharTap>(&self, input: &Input<C>) -> Result<Success<C, Self::Output>, ParseIssue> {
        self.inner.parse(input)
    }
}