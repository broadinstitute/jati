pub(crate) mod error;

use nom::IResult;
use nom_locate::LocatedSpan;
use crate::parse::error::PError;

pub(crate) type Span<'a> = LocatedSpan<&'a str>;
pub(crate) type PResult<'a, O> = IResult<Span<'a>, O, PError>;
