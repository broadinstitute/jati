use crate::error::ParseError;

pub(crate) type ParseResultOngoing<A, C>  = Result<Valid<A, C>, ParseError>;
pub(crate) type ParseResultFinal<C>  = Result<C, ParseError>;

pub enum Valid<A, C> {
    Active(A),
    Complete(C)
}

