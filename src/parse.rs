pub(crate) mod error;

use nom::IResult;
use nom_locate::LocatedSpan;
use crate::parse::error::PError;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type PResult<'a, T> = IResult<Span<'a>, T, PError>;

pub trait SParser<'a, T>: FnMut(Span<'a>) -> PResult<'a, T> {}

impl<'a, T, P> SParser<'a, T> for P
    where
        P: FnMut(Span<'a>) -> PResult<'a, T>
{}