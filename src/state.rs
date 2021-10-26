use crate::result::{ParseResultOngoing, ParseResultFinal};
use crate::failure::Failure;

pub trait State<A, C> {
    fn push_byte(&mut self, byte: u8) -> ParseResultOngoing<A, C>;
    fn push_end(&mut self) -> ParseResultFinal<C>;
    fn get_leftover_input_failure(&self, c: &C) -> Failure;
}