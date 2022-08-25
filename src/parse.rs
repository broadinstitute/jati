pub(crate) mod error;

use nom::IResult;
use nom_locate::LocatedSpan;
use crate::parse::error::PError;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type PResult<'a, O> = IResult<Span<'a>, O, PError>;
