use std::rc::Rc;
use nom::combinator::{all_consuming, map};
use nom::error::context;
use nom::sequence::delimited;
use crate::{CallParser, DefaultCallParser, IdParser, PResult, Span, SParser, Tree, WhiteSpaceParser};

pub struct ScriptParser {
    ws_parser: Rc<dyn WhiteSpaceParser>,
    id_parser: Rc<dyn IdParser>,
    call_parser: Rc<dyn CallParser>,
}

impl ScriptParser {
    pub fn new(ws_parser: Rc<dyn WhiteSpaceParser>, id_parser: Rc<dyn IdParser>) -> ScriptParser {
        let call_parser =
            Rc::new(DefaultCallParser::new(ws_parser.clone(), id_parser.clone()));
        ScriptParser { ws_parser, id_parser, call_parser }
    }
}

impl SParser<Tree> for ScriptParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Tree> {
        context("script",
                map(
                    all_consuming(
                        delimited(
                            self.ws_parser.as_fn(), self.call_parser.as_fn(),
                            self.ws_parser.as_fn(),
                        )
                    ),
                    |call| { Tree::Call(call) }),
        )(span)
    }
}

