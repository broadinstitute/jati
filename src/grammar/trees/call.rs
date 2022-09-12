use crate::grammar::trees::Tree;
use crate::grammar::trees::id::Id;

pub struct Call {
    callee: Id,
    args: Vec<Tree>,
}

impl Call {
    pub(crate) fn new(callee: Id, args: Vec<Tree>) -> Call { Call { callee, args } }
}