use crate::failure::Failure;

pub type ParseResultOngoing<A, C>  = Result<Valid<A, C>, Failure>;
pub type ParseResultFinal<C>  = Result<C, Failure>;

pub enum Valid<A, C> {
    Active(A),
    Complete(C)
}

