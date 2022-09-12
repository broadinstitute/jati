use std::rc::Rc;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::sequence::{pair, tuple};
use crate::grammar::trees::call::Call;
use crate::{PResult, Span, SParser};
use crate::grammar::trees::Tree;
use crate::parse::parsers::id::IdParser;
use crate::parse::parsers::white::WhiteSpaceParser;

pub trait CallParser: SParser<Call> {}

pub struct NoArgCallParser {
    pub(crate) ws: Rc<dyn WhiteSpaceParser>,
    pub(crate) id: Rc<dyn IdParser>,
}

impl NoArgCallParser {
    pub fn new(ws: Rc<dyn WhiteSpaceParser>, id: Rc<dyn IdParser>) -> NoArgCallParser {
        NoArgCallParser { ws, id }
    }
    pub fn ws(&self) -> Rc<dyn WhiteSpaceParser> { self.ws.clone() }
    pub fn id(&self) -> Rc<dyn IdParser> { self.id.clone() }
}

impl SParser<Call> for NoArgCallParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Call> {
        context("call",
                map(
                    tuple((
                        self.id.as_fn(), tag("("), opt(self.ws.as_fn()), tag(")"),
                        opt(pair(self.ws.as_fn(), tag(";")))
                    )),
                    |tup| {
                        let callee = tup.0;
                        let args: Vec<Tree> = Vec::new();
                        Call::new(callee, args)
                    },
                ),
        )(span)
    }
}

impl CallParser for NoArgCallParser {}
