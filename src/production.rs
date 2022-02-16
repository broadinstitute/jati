use crate::error::ParseError;

trait Production {
    type Item;
    fn item(&mut self) -> Result<Self::Item, ParseError>;
}