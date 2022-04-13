use crate::matcher::Matcher;
use crate::error::Error;

pub trait Parser<I> {
    fn compile(&self) -> Result<Matcher<I>, Error>;
}
