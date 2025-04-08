use crate::error::Error;
use crate::input::Input;
use crate::parse::{ParseIssue, Parser, Success};
use std::sync::RwLock;

pub struct DelayedInitParser<O> {
    inner: RwLock<Box<dyn Parser<Output = O>>>,
}

struct UninitializedErrorParser<O> {
    _marker: std::marker::PhantomData<O>,
}

impl<O> Parser for UninitializedErrorParser<O> {
    type Output = O;

    fn parse<'a>(&self, _: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        Err(ParseIssue::Error(Error::from(
            "Delayed init parser not initialized",
        )))
    }
}

impl<O: 'static> DelayedInitParser<O> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Box::new(UninitializedErrorParser::<O> {
                _marker: Default::default()
            })),
        }
    }

    pub fn init(&self, parser: Box<dyn Parser<Output = O>>) {
        let mut inner = self.inner.write().unwrap();
        *inner = parser;
    }
}

impl<O: 'static> Default for DelayedInitParser<O> {
    fn default() -> Self {
        Self::new()
    }
}
impl<O> Parser for DelayedInitParser<O> {
    type Output = O;

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        let inner = self.inner.read().unwrap();
        inner.parse(input)
    }
}
