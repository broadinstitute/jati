pub(crate) mod error;

use nom::IResult;
use nom_locate::LocatedSpan;
use crate::parse::error::PError;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type PResult<'a, T> = IResult<Span<'a>, T, PError>;
//pub type Parser<'a, T> = impl Fn(Span<'a>) -> PResult<'a, T>;

pub trait Parser<'a, T>: Fn(Span<'a>) -> PResult<'a, T> {}

impl<'a, T, P> Parser<'a, T> for P
    where
        P: Fn(Span<'a>) -> PResult<'a, T>
{}