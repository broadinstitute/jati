use crate::failure_old::FailureOld;

pub type ParseResultOngoing<A, C>  = Result<Valid<A, C>, FailureOld>;
pub type ParseResultFinal<C>  = Result<C, FailureOld>;

pub enum Valid<A, C> {
    Active(A),
    Complete(C)
}

