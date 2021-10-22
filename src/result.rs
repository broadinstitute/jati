use crate::failure::Failure;

pub type ParseResult<A, C> = Result<Valid<A, C>, Failure>;

pub enum Valid<A, C> {
    Active(A),
    Complete(C)
}

