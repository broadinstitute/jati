use crate::symbols::fun::FunTag;
use crate::trees::typed::tree::Tree;

pub struct Call {
    pub name: String,  // TODO: replace name with id
    pub fun: FunTag,
    pub args: Vec<Box<dyn Tree>>
}


