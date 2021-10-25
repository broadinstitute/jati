use crate::byte_state::ByteState;
use crate::result::ParseResult;

pub trait State<A, C> {
    fn push_byte_state(&mut self, byte_state: &ByteState) -> ParseResult<A, C> {
        match byte_state.to_byte_opt() {
            Some(byte) => { self.push_byte(byte) }
            None => { self.push_end() }
        }
    }
    fn push_byte(&mut self, byte: u8) -> ParseResult<A, C>;
    fn push_end(&mut self) -> ParseResult<A, C>;
}