#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct Precedence {
    precedence: u8
}

impl From<u8> for Precedence {
    fn from(precedence: u8) -> Self { Precedence { precedence } }
}

pub(crate) trait Rule<I> {
    fn precedence(&self) -> Precedence;
}
