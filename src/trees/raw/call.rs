use crate::trees::raw::tree::Tree;
use crate::trees::raw::id::Id;

pub struct Call {
    pub(crate) callee: Id,
    pub(crate) args: Vec<Tree>,
}

impl Call {
    pub(crate) fn new(callee: Id, args: Vec<Tree>) -> Call { Call { callee, args } }
}