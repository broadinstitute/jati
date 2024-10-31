use std::rc::Rc;

pub trait WhiteSpaceParser {}

pub struct DefaultWhiteSpaceParser {}

impl DefaultWhiteSpaceParser {
    pub fn new_unboxed() -> DefaultWhiteSpaceParser { DefaultWhiteSpaceParser {} }
    pub fn new() -> Rc<DefaultWhiteSpaceParser> { Rc::new(Self::new_unboxed()) }
}

impl Default for DefaultWhiteSpaceParser {
    fn default() -> Self { Self::new_unboxed() }
}

impl WhiteSpaceParser for DefaultWhiteSpaceParser {}

