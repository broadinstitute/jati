use crate::pos::Pos;
use crate::byte_state::{ByteState, ByteStateSet};

struct ParseError {
    pos: Pos,
    actual: ByteState,
    expected: ByteStateSet
}

impl ParseError {
    pub fn message(&self) -> String {
        format!("Error at {}: found {}, expected {}", self.pos, self.actual, self.expected)
    }
}