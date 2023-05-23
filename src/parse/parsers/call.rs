use std::rc::Rc;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::sequence::{pair, tuple};
use crate::trees::raw::call::Call;
use crate::{PResult, Span, SParser};
use crate::parse::parsers::id::IdParser;
use crate::parse::parsers::white::WhiteSpaceParser;
use crate::trees::raw::tree::Tree;
use crate::trees::raw::op::Op;

pub trait CallParser: SParser<Tree> {}

pub struct DefaultCallParser {
    pub(crate) ws: Rc<dyn WhiteSpaceParser>,
    pub(crate) id: Rc<dyn IdParser>,
}

impl DefaultCallParser {
    pub fn new(ws: Rc<dyn WhiteSpaceParser>, id: Rc<dyn IdParser>) -> DefaultCallParser {
        DefaultCallParser { ws, id }
    }
    pub fn ws(&self) -> Rc<dyn WhiteSpaceParser> { self.ws.clone() }
    pub fn id(&self) -> Rc<dyn IdParser> { self.id.clone() }
}

impl SParser<Tree> for DefaultCallParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Tree> {
        context("call",
                map(
                    tuple((
                        self.id.as_fn(), tag("("), self.ws.as_fn(), tag(")"),
                        opt(pair(self.ws.as_fn(), tag(";")))
                    )),
                    |tup| {
                        let id = tup.0;
                        let op = Call::new(id);
                        let args: Vec<Tree> = Vec::new();
                        op.new_tree(args).unwrap()
                    },
                ),
        )(span)
    }
}

impl CallParser for DefaultCallParser {}
