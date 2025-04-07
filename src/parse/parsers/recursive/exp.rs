use crate::error::Error;

trait B: Iterator<Item = Result<char, Error>>  {
    fn box_clone(&self) -> Box<dyn B>;
}

pub struct A {
    iter: Box<dyn B>,
}