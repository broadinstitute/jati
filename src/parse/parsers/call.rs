use std::rc::Rc;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::sequence::{pair, tuple};
use crate::{PResult, Span, SParser};
use crate::parse::parsers::id::IdParser;
use crate::parse::parsers::white::WhiteSpaceParser;
use crate::trees::tree::Tree;
use crate::trees::op::{OpExpression};
use crate::trees::op::{IdOp, Op, OpSyntax};
use crate::trees::props::Raw;

pub trait CallParser: SParser<Tree<Raw>> {}

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

impl SParser<Tree<Raw>> for DefaultCallParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Tree<Raw>> {
        context("call",
                map(
                    tuple((
                        self.id.as_fn(), tag("("), self.ws.as_fn(), tag(")"),
                        opt(pair(self.ws.as_fn(), tag(";")))
                    )),
                    |tup| {
                        let id = tup.0;
                        let syntax = OpSyntax::Call;
                        let op = Op::Id(IdOp::<Raw>::new(id, syntax));
                        let args: Vec<Tree<Raw>> = Vec::new();
                        OpExpression::new(op, args).into_tree()
                    },
                ),
        )(span)
    }
}

impl CallParser for DefaultCallParser {}
