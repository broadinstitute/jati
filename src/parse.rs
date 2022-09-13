pub(crate) mod error;
pub mod parsers;

use nom::IResult;
use nom_locate::LocatedSpan;
use crate::parse::error::PError;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type PResult<'a, T> = IResult<Span<'a>, T, PError>;

pub trait SParser<T> {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, T>;
    fn parse_str<'a>(&self, string: &'a str) -> PResult<'a, T> {
        let span = Span::new(string);
        self.parse_span(span)
    }
    fn as_fn(&self) -> Box<dyn Fn(Span) -> PResult<T> + '_> {
        Box::new(move |span| { self.parse_span(span) })
    }
}
