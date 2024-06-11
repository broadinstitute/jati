use crate::symbols::ops::OpTag;
use crate::symbols::var::VarTag;

pub trait Props {
    type VT: Clone;
    type OT: Clone;
}

pub struct Raw {}
pub struct Typed {}

impl Props for Raw {
    type VT = ();
    type OT = ();
}

impl Props for Typed {
    type VT = VarTag;
    type OT = OpTag;
}