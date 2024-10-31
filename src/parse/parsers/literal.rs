use std::rc::Rc;

pub trait LiteralParser {}

pub struct DefaultLiteralParser {}

impl DefaultLiteralParser {
    pub fn new_unboxed() -> DefaultLiteralParser { DefaultLiteralParser {} }
    pub fn new() -> Rc<DefaultLiteralParser> { Rc::new(Self::new_unboxed()) }
}

impl LiteralParser for DefaultLiteralParser {}

impl Default for DefaultLiteralParser {
    fn default() -> Self { Self::new_unboxed() }
}