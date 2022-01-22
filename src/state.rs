use crate::result::{ParseResultOngoing, ParseResultFinal};
use crate::pos::Pos;

pub(crate) trait State<A, C> {
    fn push_byte(&mut self, byte: u8) -> ParseResultOngoing<A, C>;
    fn push_end(&mut self) -> ParseResultFinal<C>;
    fn pos(&self) -> Pos;
    fn replay_unused(&mut self) -> Box<dyn Iterator<Item=u8>> {
        Box::new(std::iter::empty())
    }
}